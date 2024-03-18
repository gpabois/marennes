use std::alloc::{alloc, handle_alloc_error, Layout};

pub type PageId = usize;

/// Découple les opérations d'insertion 
/// de ceux permettant de lire ou écrire des objets gérés 
/// par le livre.
/// 
/// Cette collection :
/// - ne permet pas de supprimer un objet,
/// - gère les accès par un identifiant.
pub struct Book<const N: usize, Item>{
    pages: Vec<Page<N, Item>>
}

impl<const N: usize, Item> Book<N, Item> {
    pub fn insert(&mut self, item: Item) -> ItemId {
        let page = self.get_mut_unfilled_page();
    }

    /// Add a new page in the book
    fn add_new_page(&mut self) -> PageId {
        let page_id = self.pages.len();
        self.pages.push(Page::new(page_id));
        return page_id
    }

    /// Get a mutable unfilled page
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

/// Une ligne de la page (contient un objet, et de quoi vérifier les règles d'emprunt)
pub struct Line<Item> {
    /// Current number of borrow
    read: usize,
    /// Current number of mut borrow
    write: usize,
    /// The data
    data: Item
}

/// Identifiant de l'item stocké dans le livre.
#[derive(Clone)]
pub struct ItemId {
    page: usize,
    line: usize
}

struct InnerPage<const N: usize, Item> {
    block: [Line<Item>; N],
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

    /// Initialise the inner page
    /// Generally after allocation by Page.
    fn init(&mut self, id: usize) {
        self.rc = 0;
        self.cursor = 0;
        self.id = id;
    }

    pub(self) fn new(id: usize) -> *mut Self {
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

    /// Decrement the reference counter
    /// If zero, dealloc the page.
    pub(self) unsafe fn dec_rc(ptr: *mut InnerPage<N, Item>) {
        (*ptr).rc -= 0;
        if (*ptr).rc == 0 {
            let layout = Layout::new::<Self>();
        }
    }
}

/// Un bloc de mémoire contiguë 
/// Le bloc n'est pas déalloué tant qu'une référence a un item est maintenue.
pub struct Page<const N: usize, Item>(*mut InnerPage<N, Item>);

impl<const N: usize, Item> Page<N, Item> {
    /// Creates a new page.
    pub fn new(id: usize) -> Self {
        Self(InnerPage::new(id))
    }

    /// Returns true if there is no more room in the page
    pub fn is_full(&self) -> bool {
        unsafe {
            (*self.0).is_full()
        }
    }
}

impl<const N: usize, Item> Drop for Page<N, Item> {
    fn drop(&mut self) {
        unsafe {
            InnerPage::dec_rc(self.0)
        }
    }
}
