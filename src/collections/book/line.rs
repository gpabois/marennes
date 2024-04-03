use std::{
    cell::Cell, ops::{Deref, DerefMut}, ptr::{self, NonNull}, sync::atomic::{AtomicBool, AtomicUsize}
};

use super::{BookError, BookResult};

/// Une entrée de la page (contient un objet, et de quoi vérifier les règles d'emprunt)
pub(super) struct LineInner<Item> {
    /// Emprunt en cours en écriture
    write: AtomicBool,
    
    /// Nombre de références fortes
    strong: AtomicUsize,
    
    /// Nombre de références faibles
    weak: AtomicUsize,
    
    /// The data
    pub(self) item: Item,
}

impl<Item> Default for LineInner<Item> {
    fn default() -> Self {
        unsafe{Self::new_unitialised()}
    }
}

impl<Item> LineInner<Item> {
    /// Crée une nouvelle entrée stockant un objet.
    pub fn new(item: Item) -> Self {
        Self {
            write:  AtomicBool::new(false),
            strong: AtomicUsize::new(0),
            weak:   AtomicUsize::new(0),
            item,
        }
    }

    /// Crée une nouvelle entrée sans initialiser l'objet.
    pub unsafe fn new_unitialised() -> Self {
        Self {
            write: AtomicBool::new(false),
            strong: AtomicUsize::new(0),
            weak: AtomicUsize::new(0),
            item: std::mem::zeroed()      
        }
    }
}

/// Opérations sur les différents compteurs (Ecriture, Lecture, Forte, Faible)
impl<Item> LineInner<Item> {
    /// Acquiert un verrou en écriture
    #[inline(always)]
    pub fn acquire_write_lock(&self) -> bool {
        self.write.compare_exchange(
            false, 
            true, 
            std::sync::atomic::Ordering::Acquire, 
            std::sync::atomic::Ordering::Relaxed
        ).is_ok()
    }

    #[inline(always)]
    /// Libère le verrou en écriture
    /// Panique si aucun verrou n'existe.
    pub fn release_write_lock(&self) {
        self.write.compare_exchange(
            true, 
            false, 
            std::sync::atomic::Ordering::Release, 
            std::sync::atomic::Ordering::Relaxed
        ).expect_err("aucun verrou en écriture n'existe");
    }

    /// Acquiert un verrou en lecture
    #[inline(always)]
    pub fn acquire_read_lock(&self) -> bool {
        self.write.compare_exchange(
            false, 
            false, 
            std::sync::atomic::Ordering::Acquire, 
            std::sync::atomic::Ordering::Relaxed
        ).is_ok()
    }

    #[inline(always)]
    pub fn strong(&self) -> usize {
        self.strong.load(std::sync::atomic::Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn inc_strong(&self) -> usize {
        self.strong.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn dec_strong(&self) -> usize {
        self.strong.fetch_sub(1, std::sync::atomic::Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn weak(&self) -> usize {
        self.weak.load(std::sync::atomic::Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn inc_weak(&self) -> usize {
        self.weak.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    #[inline(always)]
    pub fn dec_weak(&self) -> usize {
        self.weak.fetch_sub(1, std::sync::atomic::Ordering::Relaxed)
    }
}

impl<Item> Drop for LineInner<Item> {
    fn drop(&mut self) {
        // Le compteur n'est jamais à zéro, 
        // sauf si toutes les références fortes ont été détruites.
        if self.weak() == 0 {
            return;
        }

        if self.strong() == 0 {           
            self.dec_weak();
        }
    }
}

/// Détruit une référence forte.
/// Si le compteur de référence forte est à zéro, détruit l'objet, 
/// et décroît le compteur de référence faible.
/// 
/// # Safety
/// La fonction doit être uniquement appelée lorsque :
/// - Line est détruit.
/// - ReadLine est détruit.
/// - Page est détruit.
pub(super) unsafe fn drop_strong_ref<Item>(mut this: NonNull<LineInner<Item>>) {
    if this.as_ref().strong() == 0 {
        return;
    }
    
    // On décrémente le compteur de référence forte, et on est à zéro.
    if this.as_ref().dec_strong() == 0 {
        // On drop l'item.
        ptr::drop_in_place(std::ptr::from_mut(&mut this.as_mut().item));
    }
}

/// Détruit une référence faible.
/// Si le compteur de référence faible est à zéro, alors
/// la ligne est considérée comme libre, et peut être réallouée
/// pour un autre usage.
pub(super) unsafe fn drop_weak_ref<Item>(ptr: NonNull<LineInner<Item>>) -> usize {
    ptr.as_ref().dec_weak()
}

/// Une référence forte et mutable sur une ligne d'une page.
pub struct Line<Item> {
    pub(super) ptr: NonNull<LineInner<Item>>
}

impl<Item> Line<Item> {
    /// Crée une nouvelle ligne, et retourne un objet mutable.
    pub(super) fn new(ptr: NonNull<LineInner<Item>>) -> Option<Self> {
        unsafe {
            // On doit s'assurer d'être le seul propriétaire pour 
            // l'instant de la ligne.
            if ptr.as_ref().inc_strong() > 1 {
                return None
            }

            // On acquiert le verrou en écriture.
            if ptr.as_ref().acquire_write_lock() {
                Some(Self{ptr})
            } else {
                None
            }
        }
    }
}

impl<Item> Line<Item> {
    /// Dégrade vers une référence faible sur la ligne.
    pub fn weak_downgrade(this: &Self) -> WeakLine<Item> {
        this.inner().inc_weak();
        WeakLine{ptr: this.ptr}
    }

    /// Transforme la référence mutable, en référence immutable.
    pub fn read_downgrade(this: Self) -> ReadLine<Item> {
        this.inner().dec_write();
        this.inner().inc_read();
        // On évite un appel à Drop sur l'objet.
        let ptr = this.ptr;
        std::mem::forget(this);
        ReadLine { ptr }
    }
}

impl<Item> Deref for Line<Item> {
    type Target = Item;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &self.ptr.as_ref().item
        }
    }
}

impl<Item> DerefMut for Line<Item> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut self.ptr.as_mut().item
        }
    }
}

impl<Item> Line<Item> {
    #[inline(always)]
    fn inner(&self) -> &LineInner<Item> {
        unsafe {self.ptr.as_ref()}
    }
}

impl<Item> Drop for Line<Item> {
    fn drop(&mut self) {
        // Libère le verrou en écriture.
        self.inner().release_write_lock();
        unsafe {
            drop_strong_ref(self.ptr);
        }
    }
}

/// Une référence forte et immutable sur une ligne d'une page.
pub struct ReadLine<Item> {
    ptr: NonNull<LineInner<Item>>
}

impl<Item> ReadLine<Item> {
    #[inline(always)]
    fn inner(&self) -> &LineInner<Item> {
        unsafe {self.ptr.as_ref()}
    }
}

impl<Item> ReadLine<Item> {
    /// Dégrade vers une référence faible sur la ligne.
    pub fn weak_downgrade(this: &Self) -> WeakLine<Item> {
        this.inner().inc_weak();
        WeakLine{ptr: this.ptr}
    }

    /// Essaye de consumer la référence immutable, pour
    /// produire une référence mutable, si et seulement si
    /// aucun autre emprunt existe.
    pub fn try_write_upgrade(this: Self) -> BookResult<Line<Item>> {
        // On a plus d'une référence forte, impossible.
        if this.inner().strong() > 1 {
            return Err(BookError::AlwaysBorrowed);
        }

        // On essaye d'acquérir un verrou en écriture.
        if !this.inner().acquire_write_lock() {
            return Err(BookError::AlwaysMutBorrowed);
        }

        let ptr = this.ptr;
        std::mem::forget(this);
        Ok(Line{ptr})
    }
}

impl<Item> Deref for ReadLine<Item> {
    type Target = Item;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &self.ptr.as_ref().item
        }
    }
}

impl<Item> Drop for ReadLine<Item> {
    fn drop(&mut self) {
        unsafe{drop_strong_ref(self.ptr);}
    }
}

/// Référence faible vers une ligne d'une page.
pub struct WeakLine<Item> {
    ptr: NonNull<LineInner<Item>>
}

impl<Item> Clone for WeakLine<Item> {
    fn clone(&self) -> Self {
        self.inner().inc_weak();
        Self { ptr: self.ptr.clone() }
    }
}

impl<Item> WeakLine<Item> {
    /// Essaye d'obtenir une référence mutable vers l'entrée.
    /// 
    /// Retourne:
    /// - None si l'objet a été détruit.
    /// - Err(BookError::AlwaysBorrowed), si une référence immutable existe.
    /// - Err(BookError::AlwaysMutBorrowed), si une référence mutable existe.
    /// - la référence mutable sinon.
    pub fn try_write_upgrade(&self) -> Option<BookResult<Line<Item>>> {
        // L'objet a été libéré...
        if self.inner().strong() == 0 {
            return None;
        }

        self.inner().inc_strong();

        // On a pas réussi à attraper un verrou en écriture.
        if !self.inner().acquire_write_lock() {
            self.inner().dec_strong();
            return Some(Err(BookError::AlwaysBorrowed));
        }

        Some(Ok(Line{ptr: self.ptr}))
    }

    /// Essaye d'obtenir une référence immutable vers l'entrée.
    /// 
    /// Retourne:
    /// - None si l'objet a été détruit.
    /// - Err(BookError::AlwaysMutBorrowed), si une référence mutable existe.
    /// - la référence immutable sinon.
    pub fn try_read_upgrade(&self) -> Option<BookResult<ReadLine<Item>>> {
        // L'objet a été libéré...
        if self.inner().strong() == 0 {
            return None;
        }

        self.inner().inc_strong();
        if !self.inner().acquire_read_lock() {
            self.inner().dec_strong();
            return Some(Err(BookError::AlwaysMutBorrowed));
        }

        Some(Ok(ReadLine{ptr: self.ptr}))
    }
}

impl<Item> WeakLine<Item> {
    #[inline(always)]
    fn inner(&self) -> &LineInner<Item> {
        unsafe{self.ptr.as_ref()}
    }
}

impl<Item> Drop for WeakLine<Item> {
    fn drop(&mut self) {
        unsafe{drop_weak_ref(self.ptr);}
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::NonNull;

    use super::{Line, LineInner};

    #[test]
    fn test_simple_line() {
        // On crée une nouvelle ligne interne sur la stack.
        let mut inner = LineInner::new(10);
        let ptr = unsafe {
            NonNull::new_unchecked(
                std::ptr::from_mut(&mut inner)
            )
        };
        
        // On crée une référence en écriture sur l'objet.
        let s1 = Line::new(ptr);

        // On doit avoir une référence forte comptée.
        assert_eq!(inner.strong(), 1);
        // On doit avoir une référence faible = 1 pour n références fortes.
        assert_eq!(inner.weak(), 1);
        
        drop(s1);

        assert_eq!(inner.strong(), 0);
        assert_eq!(inner.weak(), 0);
    }
}