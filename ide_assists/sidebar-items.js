initSidebarItems({"enum":[["AssistKind",""],["AssistResolveStrategy","A way to control how many asssist to resolve during the assist resolution. When an assist is resolved, its edits are calculated that might be costly to always do by default."]],"mod":[["ast_transform","`AstTransformer`s are functions that replace nodes in an AST and can be easily combined."],["utils","Assorted functions shared by several assists."]],"struct":[["Assist",""],["AssistConfig",""],["AssistId","Unique identifier of the assist, should not be shown to the user directly."],["GroupLabel",""],["SingleResolve","Hold the [`AssistId`] data of a certain assist to resolve. The original id object cannot be used due to a `'static` lifetime and the requirement to construct this struct dynamically during the resolve handling."]]});