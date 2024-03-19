use std::{
    alloc::{alloc, dealloc, handle_alloc_error, Layout},
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub enum BookError {
    AlreadyBorrowed,
    AlreadyMutBorrowed,
}

pub type BookResult<D> = Result<D, BookError>;

type PageId = usize;

/// Identifiant d'un objet stocké dans un livre.
#[derive(Clone)]
pub struct BookItemId {
    page: usize,
    line: usize,
}

/// Découple les opérations d'insertion
/// de ceux permettant de lire ou écrire des objets gérés
/// par le livre.
///
/// Cette collection :
/// - ne permet pas de supprimer un objet,
/// - gère les accès par un identifiant.
pub struct Book<const N: usize, Item>
where
    Item: 'static,
{
    pages: Vec<Page<N, Item>>,
}

impl<const N: usize, Item> Default for Book<N, Item> {
    fn default() -> Self {
        Self {
            pages: Default::default(),
        }
    }
}

impl<const N: usize, Item> Book<N, Item>
where
    Item: 'static,
{
    /// Ecris une nouvelle entrée dans le livre.
    pub fn write(&mut self, item: Item) -> BookItemId {
        let page = self.get_mut_unfilled_page();
        page.write(item).unwrap()
    }

    /// Récupére une référence sur un objet à partir de son identifiant.
    ///
    /// Retourne None si aucun objet existe
    /// Retourne une erreur si l'objet est déjà mut-emprunté.
    /// Retourne une référence si tout est bon.
    pub fn try_get(&self, id: &BookItemId) -> Option<Result<RefBookEntry<'_, Item>, BookError>> {
        self.pages.get(id.page).and_then(|page| page.try_get(id))
    }

    /// Récupére une référence sur un objet, panique en cas d'erreur.
    pub fn get(&self, id: &BookItemId) -> RefBookEntry<'_, Item> {
        self.try_get(id).unwrap().unwrap()
    }

    /// Récupére une mut-référence sur un object à partir de son identifiant.
    ///
    /// Retourne None si aucun objet existe
    /// Retourne une erreur si l'object est déjà mut-empruntée ou juste empruntée.
    /// Retourne une référence mutable si tout est bon.
    pub fn try_get_mut(&self, id: &BookItemId) -> Option<BookResult<MutBookEntry<'_, Item>>> {
        self.pages
            .get(id.page)
            .and_then(|page| page.try_get_mut(id))
    }

    pub fn get_mut(&self, id: &BookItemId) -> MutBookEntry<'_, Item> {
        self.try_get_mut(id).unwrap().unwrap()
    }

    /// Alloue une nouvelle entrée dans le livre.
    ///
    /// # Safety
    /// Cette opération n'est pas sûre car elle n'initialise pas l'entrée.
    pub unsafe fn alloc_entry(&mut self) -> BookItemId {
        let page = self.get_mut_unfilled_page();
        page.alloc().unwrap()
    }

    /// Initialise une entrée dans le livre.
    ///
    /// # Safety
    /// Cette opération n'est pas sûre car elle va réécrire une entrée qui est peut être déjà en
    /// cours d'emprunt.
    pub unsafe fn init_entry(&mut self, id: &BookItemId, item: Item) {
        if let Some(page) = self.pages.get_mut(id.page) {
            page.init_entry(id, item);
        }
    }

    /// Ajoute une nouvelle page au livre.
    fn add_new_page(&mut self) -> PageId {
        let page_id = self.pages.len();
        self.pages.push(Page::new(page_id));
        page_id
    }

    /// Récupère une mut-référence sur une page qui est garantie de ne pas être pleine.
    fn get_mut_unfilled_page(&mut self) -> &mut Page<N, Item> {
        // No page in the book, yet.
        if self.pages.is_empty() {
            let page_id = self.add_new_page();
            return self.pages.get_mut(page_id).unwrap();
        }

        // Last page has no more room to spare.
        if self.pages.last().unwrap().is_full() {
            let page_id = self.add_new_page();
            return self.pages.get_mut(page_id).unwrap();
        }

        // Return the last page in the book
        self.pages.last_mut().unwrap()
    }
}

/// Une entrée de la page (contient un objet, et de quoi vérifier les règles d'emprunt)
struct Entry<Item> {
    /// Current number of borrow
    read: usize,
    /// Current number of mut borrow
    write: usize,
    /// The data
    item: Item,
}

impl<Item> Entry<Item> {
    /// Crée une nouvelle entrée stockant un objet.
    pub fn new(item: Item) -> Self {
        Self {
            read: 0,
            write: 0,
            item,
        }
    }
}

/// Structure de donnée interne à une page
///
/// Elle est allouée sur le tas en interne par Page.
/// Elle est partagée via PageRef pour éviter une déallocation.
struct InnerPage<const N: usize, Item> {
    block: [Entry<Item>; N],
    /// L'identifiant de l'emplacement libre
    cursor: usize,
    /// Compteur de référence
    rc: usize,
    /// Identifiant de la page
    id: usize,
}

impl<const N: usize, Item> InnerPage<N, Item> {
    pub(self) fn is_full(&self) -> bool {
        self.cursor >= N
    }

    /// Essaye d'acquérir une référence d'une entrée dans la page.
    ///
    /// Retourne None si aucune entrée avec cet identifiant existe.
    /// Retourne une erreur si l'objet est déjà mut-empruntée
    pub fn acquire_entry_ref(&mut self, id: &BookItemId) -> Option<Result<&Item, BookError>> {
        if self.cursor <= id.line {
            return None;
        }

        let line = &mut self.block[id.line];
        // On a déjà acquis un verrou en écriture sur la ligne.
        if line.write > 0 {
            return Some(Err(BookError::AlreadyMutBorrowed));
        }

        line.read += 1;
        Some(Ok(&line.item))
    }

    /// Libére la référence sur l'entrée.
    pub fn release_entry_ref(&mut self, id: &BookItemId) {
        let line = &mut self.block[id.line];
        line.read -= 1;
    }

    /// Essaye d'acquérir une mut-référence d'une entrée dans la page.
    /// Retourne None si aucune entrée n'existe.
    /// Retourne une erreur si l'objet est déjà mut-empruntée, ou juste empruntée.
    pub fn acquire_entry_mut(&mut self, id: &BookItemId) -> Option<Result<&mut Item, BookError>> {
        if self.cursor <= id.line {
            return None;
        }

        let line = &mut self.block[id.line];
        // On a déjà acquis un verrou en écriture sur la ligne.
        if line.read > 0 {
            return Some(Err(BookError::AlreadyBorrowed));
        }
        if line.write > 0 {
            return Some(Err(BookError::AlreadyMutBorrowed));
        }

        line.write += 1;
        Some(Ok(&mut line.item))
    }

    pub fn release_entry_mut(&mut self, id: &BookItemId) {
        let line = &mut self.block[id.line];
        line.write -= 1;
    }

    /// Ecris une nouvelle entrée dans la page, et retourne son identifiant.
    ///
    /// Si il n'y a plus de place, retourne None.
    /// Sinon, retourne l'identifiant de l'entrée tout juste écrite.
    pub(self) fn write(&mut self, item: Item) -> Option<BookItemId> {
        if self.is_full() {
            return None;
        }

        let line_id = self.cursor;
        self.cursor += 1;

        self.block[line_id] = Entry::new(item);

        Some(BookItemId {
            page: self.id,
            line: line_id,
        })
    }

    /// Alloue une nouvelle entrée dans une page et retourne un idenfiant.
    /// Cette fonction n'est pas sûre car il n'initialise pas l'entrée.
    pub unsafe fn alloc(&mut self) -> Option<BookItemId> {
        if self.is_full() {
            return None;
        }

        let line_id = self.cursor;
        self.cursor += 1;

        Some(BookItemId {
            page: self.id,
            line: line_id,
        })
    }

    /// Initialise une entrée
    /// Cette fonction n'est pas sûre car elle peut écraser des emprunts en cours.
    pub unsafe fn init_entry(&mut self, id: &BookItemId, item: Item) {
        self.block[id.line] = Entry::new(item);
    }

    /// Initialise la page.
    ///
    /// Cette fonction est appelée après une allocation par new.
    fn init(&mut self, id: usize) {
        self.rc = 0;
        self.cursor = 0;
        self.id = id;
    }

    /// Alloue une nouvelle page sur le tas.
    ///
    /// Execute également l'initialisation.
    pub fn new(id: usize) -> *mut Self {
        unsafe {
            let layout = Layout::new::<InnerPage<N, Item>>();
            let ptr = alloc(layout) as *mut InnerPage<N, Item>;
            if ptr.is_null() {
                handle_alloc_error(layout);
            }

            // Initialise the page data.
            (*ptr).init(id);

            ptr
        }
    }

    /// Acquiert une référence sur la page.
    ///
    /// Cela empêche une déallocation de la page tant que le compteur de référence est supérieur à
    /// zéro.
    ///
    /// Tout acquisition doit faire l'objet d'une libération.
    pub unsafe fn acquire(ptr: *mut InnerPage<N, Item>) {
        (*ptr).rc += 1;
    }

    /// Libère une référence sur la page
    ///
    /// Si le compteur est à zéro, la page est déallouée.
    /// Toute libération doit être associée à une acquisition.
    pub unsafe fn release(ptr: *mut InnerPage<N, Item>) {
        (*ptr).rc -= 0;
        if (*ptr).rc == 0 {
            let layout = Layout::new::<Self>();
            dealloc(ptr as *mut u8, layout);
        }
    }
}

/// Un bloc de mémoire contiguë, servant à stocker des entrées.
///
/// La page n'est pas déallouée tant qu'une référence a un objet est maintenue.
struct Page<const N: usize, Item>(*mut InnerPage<N, Item>);

impl<const N: usize, Item> Page<N, Item>
where
    Item: 'static,
{
    /// Creates a new page.
    pub fn new(id: usize) -> Self {
        Self(InnerPage::new(id))
    }

    /// Returns true if there is no more room in the page
    pub fn is_full(&self) -> bool {
        unsafe { (*self.0).is_full() }
    }

    pub fn try_get(&self, id: &BookItemId) -> Option<BookResult<RefBookEntry<'_, Item>>> {
        unsafe {
            (*self.0).acquire_entry_ref(id).map(|maybe_entry| {
                maybe_entry.map(|entry| RefBookEntry {
                    page: PageDriver::new(self.0),
                    id: id.clone(),
                    entry,
                })
            })
        }
    }

    pub fn try_get_mut(&self, id: &BookItemId) -> Option<BookResult<MutBookEntry<'_, Item>>> {
        unsafe {
            (*self.0).acquire_entry_mut(id).map(|maybe_entry| {
                maybe_entry.map(|entry| MutBookEntry {
                    page: PageDriver::new(self.0),
                    id: id.clone(),
                    entry,
                })
            })
        }
    }

    pub fn write(&mut self, item: Item) -> Option<BookItemId> {
        unsafe { (*self.0).write(item) }
    }

    /// Alloue une nouvelle entrée dans la page.
    pub unsafe fn alloc(&mut self) -> Option<BookItemId> {
        (*self.0).alloc()
    }

    pub unsafe fn init_entry(&mut self, id: &BookItemId, item: Item) {
        (*self.0).init_entry(id, item)
    }
}

impl<const N: usize, Item> Drop for Page<N, Item> {
    fn drop(&mut self) {
        unsafe { InnerPage::release(self.0) }
    }
}

/// Pilote déporté pour gérer les libérations sur une page.
/// Le type permet d'effacer la constante N du nombre d'entrées dans une page.
struct PageDriver {
    release: Box<dyn Fn()>,
    release_mut_ref: Box<dyn Fn(&BookItemId)>,
    release_ref: Box<dyn Fn(&BookItemId)>,
}

impl PageDriver {
    pub fn new<const N: usize, Item>(ptr: *mut InnerPage<N, Item>) -> Self
    where
        Item: 'static,
    {
        unsafe {
            InnerPage::acquire(ptr);
            Self {
                release: Box::new(move || {
                    InnerPage::release(ptr);
                }),
                release_mut_ref: Box::new(move |id| {
                    (*ptr).release_entry_mut(id);
                }),
                release_ref: Box::new(move |id| {
                    (*ptr).release_entry_ref(id);
                }),
            }
        }
    }
}

impl Drop for PageDriver {
    fn drop(&mut self) {
        (*self.release)();
    }
}

/// Une mut-référence à une entrée stockée dans un livre.
pub struct MutBookEntry<'a, Item> {
    page: PageDriver,
    id: BookItemId,
    entry: &'a mut Item,
}

impl<'a, Item> Deref for MutBookEntry<'a, Item> {
    type Target = Item;

    fn deref(&self) -> &Self::Target {
        self.entry
    }
}

impl<'a, Item> DerefMut for MutBookEntry<'a, Item> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.entry
    }
}

impl<'a, Item> Drop for MutBookEntry<'a, Item> {
    fn drop(&mut self) {
        (*self.page.release_mut_ref)(&self.id)
    }
}

/// Une référence à un objet stocké dans un livre.
pub struct RefBookEntry<'a, Item> {
    page: PageDriver,
    id: BookItemId,
    entry: &'a Item,
}

impl<'a, Item> Deref for RefBookEntry<'a, Item> {
    type Target = Item;

    fn deref(&self) -> &Self::Target {
        self.entry
    }
}

impl<'a, Item> Drop for RefBookEntry<'a, Item> {
    fn drop(&mut self) {
        (*self.page.release_ref)(&self.id);
    }
}

#[cfg(test)]
mod tests {
    use super::Book;

    #[test]
    fn test_write_overflow() {
        let mut book = Book::<2, usize>::default();

        let i1 = book.write(1);
        let i2 = book.write(2);
        let i3 = book.write(3);

        assert_eq!(*book.get(&i1), 1);
        assert_eq!(*book.get(&i2), 2);
        assert_eq!(*book.get(&i3), 3);
    }

    #[test]
    fn test_fail_get_mut_if_already_borrowed() {
        let mut book = Book::<2, usize>::default();

        let i1 = book.write(1);

        let _rf = book.get(&i1);
        let res_mut = book.try_get_mut(&i1).unwrap();

        assert!(res_mut.is_err());
    }

    #[test]
    fn test_fail_get_if_already_mut_borrowed() {
        let mut book = Book::<2, usize>::default();

        let i1 = book.write(1);

        let _rf = book.get_mut(&i1);
        let res_mut = book.try_get(&i1).unwrap();

        assert!(res_mut.is_err());
    }
}
