#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
pub mod info;
use editor::Editor;
pub use terminal::Terminal;

fn main() {
    let mut editor = Editor::default();

    editor.run();
}


// 赶紧记录一下，我的需求是把info取得terminal，但是在info那个文件，不可导入。然后在main这里的pub Terminal 他有的，所以在导入带info也给他代入了，
// 用作用域的意思去理解。

// 还有可以直接 C-w + s/v 来加buffer视窗    
