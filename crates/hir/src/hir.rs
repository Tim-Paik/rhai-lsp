mod add;
pub mod query;
pub mod resolve;

use core::ops;

use crate::{
    module::{ModuleData, ModuleKind},
    scope::ScopeData,
    source::{Source, SourceData},
    symbol::*,
    Module, Scope,
};

use rhai_rowan::syntax::SyntaxNode;
use slotmap::{Key, SlotMap};
use url::Url;

#[derive(Debug, Default, Clone)]
pub struct Hir {
    static_module: Module,
    modules: SlotMap<Module, ModuleData>,
    scopes: SlotMap<Scope, ScopeData>,
    symbols: SlotMap<Symbol, SymbolData>,
    sources: SlotMap<Source, SourceData>,
}

static_assertions::assert_impl_all!(Hir: Send, Sync);

impl Hir {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Hir {
    pub fn clear(&mut self) {
        self.symbols.clear();
        self.scopes.clear();
        self.modules.clear();
        self.sources.clear();
        self.static_module = Module::default();
    }

    #[must_use]
    pub fn symbol(&self, symbol: Symbol) -> Option<&SymbolData> {
        self.symbols.get(symbol)
    }

    pub fn symbols(&self) -> impl Iterator<Item = (Symbol, &SymbolData)> {
        self.symbols.iter()
    }

    #[must_use]
    pub fn scope(&self, scope: Scope) -> Option<&ScopeData> {
        self.scopes.get(scope)
    }

    pub fn scopes(&self) -> impl Iterator<Item = (Scope, &ScopeData)> {
        self.scopes.iter()
    }

    #[must_use]
    pub fn module(&self, module: Module) -> Option<&ModuleData> {
        self.modules.get(module)
    }

    pub fn modules(&self) -> impl Iterator<Item = (Module, &ModuleData)> {
        self.modules.iter()
    }

    pub fn sources(&self) -> impl Iterator<Item = (Source, &SourceData)> {
        self.sources.iter()
    }

    #[must_use]
    pub fn source_for(&self, url: &Url) -> Option<Source> {
        self.sources()
            .find_map(|(s, data)| if data.url == *url { Some(s) } else { None })
    }

    fn symbol_mut(&mut self, symbol: Symbol) -> &mut SymbolData {
        self.symbols.get_mut(symbol).unwrap()
    }

    fn scope_mut(&mut self, scope: Scope) -> &mut ScopeData {
        self.scopes.get_mut(scope).unwrap()
    }

    #[allow(dead_code)]
    fn module_mut(&mut self, module: Module) -> &mut ModuleData {
        self.modules.get_mut(module).unwrap()
    }

    fn source_mut(&mut self, source: Source) -> &mut SourceData {
        self.sources.get_mut(source).unwrap()
    }

    // pub fn resolve_references(&mut self) {
    //     for (_, m) in self.modules.iter_mut() {
    //         m.resolve_references();
    //     }
    // }

    // pub fn resolve_references_in_module(&mut self, name: &str) {
    //     if let Some(m) = self.modules.get_mut(name) {
    //         m.resolve_references();
    //     }
    // }

    // pub fn infer_types(&mut self) {
    //     for (_, m) in self.modules.iter_mut() {
    //         m.infer_types();
    //     }
    // }
}

impl ops::Index<Scope> for Hir {
    type Output = ScopeData;

    fn index(&self, index: Scope) -> &Self::Output {
        self.scopes.get(index).unwrap()
    }
}

impl ops::Index<Symbol> for Hir {
    type Output = SymbolData;

    fn index(&self, index: Symbol) -> &Self::Output {
        self.symbols.get(index).unwrap()
    }
}

impl ops::Index<Module> for Hir {
    type Output = ModuleData;

    fn index(&self, index: Module) -> &Self::Output {
        self.modules.get(index).unwrap()
    }
}

impl ops::Index<Source> for Hir {
    type Output = SourceData;

    fn index(&self, index: Source) -> &Self::Output {
        self.sources.get(index).unwrap()
    }
}
