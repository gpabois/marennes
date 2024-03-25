use std::ops::{Deref, DerefMut};

use super::{line::{Line, ReadLine, WeakLine}, page::{Page, WeakPage}, BookResult};

/// Une référence mutable vers une entrée dans un livre.
/// 
/// Cet objet maintient une référence forte sur la page, et la ligne
/// de sorte que ni l'un, ni l'autre ne peuvent être libérées.
/// 
/// Depuis cet objet deux choses peuvent être faites :
/// - dégrader vers une référence forte immutable ;
/// - dégrader vers une référence faible.
pub struct BookEntry<const N: usize, Item> {
    /// Une référence forte vers la page.
    pub(super) page: Page<N, Item>,
    /// Une référence forte vers la ligne.
    pub(super) line: Line<Item>
}

impl<const N: usize, Item> BookEntry<N, Item> {
    /// Dégrade vers une référence faible.
    pub fn weak_downgrade(this: &Self) -> BookWeakEntry<N, Item> {
        BookWeakEntry {
            line: Line::weak_downgrade(&this.line),
            page: Page::downgrade(&this.page)
        }
    }
}

impl<const N: usize, Item> Deref for BookEntry<N, Item> {
    type Target = Item;

    fn deref(&self) -> &Self::Target {
        self.line.deref()
    }
}

impl<const N: usize, Item> DerefMut for BookEntry<N, Item> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.line.deref_mut()
    }
}

pub struct BookReadEntry<const N: usize, Item> {
    /// Une référence forte vers la page.
    pub(super) page: Page<N, Item>,
    /// Une référence forte vers la ligne.
    pub(super) line: ReadLine<Item>  
}

/// On libère la ligne, puis la page.
impl<const N: usize, Item> Drop for BookEntry<N, Item> {
    fn drop(&mut self) {
        drop(self.line);
        drop(self.page);
    }
}

/// Une référence faible vers une entrée dans un livre.
pub struct BookWeakEntry<const N: usize, Item> {
    page: WeakPage<N, Item>,
    line: WeakLine<Item>
}

impl<const N: usize, Item> Clone for BookWeakEntry<N, Item> {
    fn clone(&self) -> Self {
        Self { page: self.page.clone(), line: self.line.clone() }
    }
}

impl<const N: usize, Item> BookWeakEntry<N, Item> {
    /// Upgrade la référence faible vers une référence mutable.
    /// 
    /// Retourne:
    /// - None si l'objet a été détruit.
    /// - Err(BookError::AlwaysBorrowed), si une référence immutable existe.
    /// - Err(BookError::AlwaysMutBorrowed), si une référence mutable existe.
    /// - la référence mutable sinon.
    pub fn try_write_upgrade(&self) -> Option<BookResult<BookEntry<N, Item>>> {
        // On doit déjà garantir de bloquer une référence forte sur la page.
        if let Some(page) = self.page.try_upgrade() {
            return self.line.try_write_upgrade()
            .map(|maybe_line|
                 maybe_line.map(|line| 
                    BookEntry {page, line}
                )
            )
        }
        
        None
    }

    /// Upgrade la référence faible vers une référence immutable.
    /// 
    /// Retourne:
    /// - None si l'objet a été détruit.
    /// - Err(BookError::AlwaysMutBorrowed), si une référence mutable existe.
    /// - la référence mutable sinon.
    pub fn try_read_upgrade(&self) -> Option<BookResult<BookReadEntry<N, Item>>> {
        // On doit déjà garantir de bloquer une référence forte sur la page.
        if let Some(page) = self.page.try_upgrade() {
            return self.line.try_read_upgrade()
            .map(|maybe_line|
                 maybe_line.map(|line| 
                    BookReadEntry {page, line}
                )
            )
        }
        
        None
    }
}

/// On libère la ligne, puis la page.
impl<const N: usize, Item> Drop for BookWeakEntry<N, Item> {
    fn drop(&mut self) {
        drop(self.line);
        drop(self.page);
    }
}