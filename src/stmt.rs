use crate::algorithm::Printer;
use syn::{Expr, Stmt};

impl Printer {
    pub fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Local(local) => {
                self.outer_attrs(&local.attrs);
                self.ibox(0);
                self.word("let ");
                self.pat(&local.pat);
                if let Some((_eq, init)) = &local.init {
                    self.word(" = ");
                    self.neverbreak();
                    self.expr(init);
                }
                self.word(";");
                self.end();
                self.hardbreak();
            }
            Stmt::Item(item) => self.item(item),
            Stmt::Expr(expr) => {
                self.ibox(0);
                self.expr(expr);
                self.end();
                self.hardbreak();
            }
            Stmt::Semi(expr, _semi) => {
                if let Expr::Verbatim(tokens) = expr {
                    if tokens.is_empty() {
                        return;
                    }
                }
                self.ibox(0);
                self.expr(expr);
                if !meaningless_semi(expr) {
                    self.word(";");
                }
                self.end();
                self.hardbreak();
            }
        }
    }
}

fn meaningless_semi(expr: &Expr) -> bool {
    match expr {
        Expr::ForLoop(_) | Expr::While(_) => true,
        Expr::Group(group) => meaningless_semi(&group.expr),
        Expr::If(expr) => match &expr.else_branch {
            Some((_else_token, else_branch)) => meaningless_semi(else_branch),
            None => true,
        },
        _ => false,
    }
}
