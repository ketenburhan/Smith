use super::{CharIter, Editable, LineIter, Modifiable, Movement, Named, Saveable, Undoable};
use delegate_attr::delegate;
use std::io::Result;

pub type Selection = (usize, usize);

pub trait Selectable {
    fn sel(&self) -> &Option<Selection>;
    fn set_sel(&mut self, selection: Selection);
    fn reset_sel(&mut self);
    fn in_sel(&self, pos: usize) -> bool {
        match *self.sel() {
            Some((beg, end)) => pos >= beg && pos <= end,
            None => false,
        }
    }
}

pub struct Select<T>
where
    T: Editable,
{
    content: T,
    sel: Option<Selection>,
}

impl<T> Select<T>
where
    T: Editable,
{
    pub fn new(content: T) -> Select<T> {
        Select { content, sel: None }
    }
}

impl<T> Selectable for Select<T>
where
    T: Editable,
{
    fn sel(&self) -> &Option<Selection> {
        &self.sel
    }

    fn set_sel(&mut self, selection: Selection) {
        self.sel = Some(selection);
    }

    fn reset_sel(&mut self) {
        self.sel = None;
    }
}

#[delegate(self.content)]
impl<T> Editable for Select<T>
where
    T: Editable,
{
    fn step(&mut self, mov: Movement) -> ();
    fn move_to(&mut self, pos: usize) -> ();
    fn move_at(&mut self, line: usize, col: usize) -> ();
    fn insert(&mut self, c: char) -> ();
    fn insert_forward(&mut self, c: char) -> ();
    fn delete(&mut self) -> Option<char>;
    fn delete_forward(&mut self) -> Option<char>;
    fn pos(&self) -> usize;
    fn line(&self) -> usize;
    fn col(&self) -> usize;
    fn line_count(&self) -> usize;
    fn len(&self) -> usize;
    fn iter(&self) -> CharIter;
    fn lines(&self) -> LineIter;
    fn iter_line(&self, line: usize) -> CharIter;
    fn line_index_to_char_index(&self, line: usize) -> usize;
}

#[delegate(self.content)]
impl<T> Saveable for Select<T>
where
    T: Editable + Saveable,
{
    fn save(&mut self) -> Result<()>;
}

#[delegate(self.content)]
impl<T> Named for Select<T>
where
    T: Editable + Named,
{
    fn name(&self) -> &String;
    fn set_name(&mut self, name: String) -> ();
}

#[delegate(self.content)]
impl<T> Undoable for Select<T>
where
    T: Editable + Undoable,
{
    fn undo(&mut self) -> ();
    fn redo(&mut self) -> ();
    fn history_len(&self) -> usize;
}

#[delegate(self.content)]
impl<T> Modifiable for Select<T>
where
    T: Editable + Modifiable,
{
    fn was_modified(&self) -> bool;
}
