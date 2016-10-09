#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    // table name, body
    AlterTable(QualifiedName, AlterTableBody),
    // object name
    Analyze(Option<QualifiedName>),
    Attach {
        // TODO distinction between ATTACH and ATTACH DATABASE
        expr: Expr,
        db_name: Expr,
        key: Option<Expr>,
    },
    // tx type, tx name
    Begin(Option<TransactionType>, Option<Name>),
    // tx name
    Commit(Option<Name>), // TODO distinction between COMMIT and END
    CreateIndex {
        unique: bool,
        if_not_exists: bool,
        idx_name: QualifiedName,
        tbl_name: Name,
        columns: Vec<SortedColumn>,
        where_clause: Option<Expr>,
    },
    CreateTable {
        temporary: bool, // TODO distinction between TEMP and TEMPORARY
        if_not_exists: bool,
        tbl_name: QualifiedName,
        body: CreateTableBody,
    },
    CreateTrigger {
        temporary: bool,
        if_not_exists: bool,
        trigger_name: QualifiedName,
        time: Option<TriggerTime>,
        event: TriggerEvent,
        tbl_name: Option<QualifiedName>,
        for_each_row: bool,
        when_clause: Option<Expr>,
        commands: Vec<TriggerCmd>,
    },
    CreateView {
        temporary: bool,
        if_not_exists: bool,
        view_name: QualifiedName,
        columns: Option<Vec<IndexedColumn>>,
        select: Select,
    },
    CreateVirtualTable {
        if_not_exists: bool,
        tbl_name: QualifiedName,
        module_name: Name,
        args: Option<Vec<String>>,
    },
    Delete {
        with: Option<With>,
        tbl_name: QualifiedName,
        indexed: Option<Indexed>,
        where_clause: Option<Expr>,
        order_by: Option<Vec<SortedColumn>>,
        limit: Option<Limit>,
    },
    // db name
    Detach(Expr), // TODO distinction between DETACH and DETACH DATABASE
    DropIndex {
        if_exists: bool,
        idx_name: QualifiedName,
    },
    DropTable {
        if_exists: bool,
        tbl_name: QualifiedName,
    },
    DropTrigger {
        if_exists: bool,
        trigger_name: QualifiedName,
    },
    DropView {
        if_exists: bool,
        view_name: QualifiedName,
    },
    Insert {
        with: Option<With>,
        or_conflict: Option<ResolveType>, // TODO distinction between REPLACE and INSERT OR REPLACE
        tbl_name: QualifiedName,
        columns: Option<Vec<Name>>,
        body: InsertBody,
    },
    // pragma name, body
    Pragma(QualifiedName, Option<PragmaBody>),
    Reindex { obj_name: Option<QualifiedName> },
    // savepoint name
    Release(Option<Name>), // TODO distinction between RELEASE and RELEASE SAVEPOINT
    Rollback {
        tx_name: Option<Name>,
        savepoint_name: Option<Name>, // TODO distinction between TO and TO SAVEPOINT
    },
    // savepoint name
    Savepoint(Option<Name>),
    Select(Select),
    Update {
        with: Option<With>,
        or_conflict: Option<ResolveType>,
        tbl_name: QualifiedName,
        indexed: Option<Indexed>,
        sets: Vec<Set>,
        where_clause: Option<Expr>,
        order_by: Option<Vec<SortedColumn>>,
        limit: Option<Limit>,
    },
    // database name
    Vacuum(Option<Name>),
}

// TODO
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Select {
    pub with: Option<With>,
    pub body: SelectBody,
    pub order_by: Option<Vec<SortedColumn>>,
    pub limit: Option<Limit>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectBody {
    pub select: OneSelect,
    pub compounds: Option<Vec<CompoundSelect>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompoundSelect {
    pub operator: CompoundOperator,
    pub select: OneSelect,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CompoundOperator {
    Union,
    UnionAll,
    Except,
    Intersect,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OneSelect {
    Select {
        distinctness: Option<Distinctness>,
        columns: Vec<ResultColumn>,
        from: Option<FromClause>,
        where_clause: Option<Expr>,
        group_by: Option<GroupBy>,
    },
    Values(Vec<Vec<Expr>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FromClause {
    pub select: Box<SelectTable>,
    pub joins: Option<Vec<JoinedSelectTable>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Distinctness {
    Distinct,
    All,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResultColumn {
    Expr(Expr, Option<As>),
    Star,
    // table name
    TableStar(Name),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum As {
    As(Name),
    Elided(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinedSelectTable {
    pub operator: JoinOperator,
    pub table: SelectTable,
    pub constraint: Option<JoinConstraint>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectTable {
    Table(QualifiedName, Option<As>, Option<Indexed>),
    TableCall(QualifiedName, Option<Vec<Expr>>, Option<As>),
    Select(Select, Option<As>),
    Sub(FromClause, Option<As>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JoinOperator {
    Comma,
    Join,
    TypedJoin { natural: bool, join_type: JoinType },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JoinType {
    Left,
    LeftOuter,
    Inner,
    Cross,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JoinConstraint {
    On(Expr),
    // col names
    Using(Vec<Name>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupBy {
    pub exprs: Vec<Expr>,
    pub having: Option<Expr>,
}

pub type Name = String; // TODO

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QualifiedName {
    pub db_name: Option<Name>,
    pub name: Name,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlterTableBody {
    // new table name
    RenameTo(Name),
    AddColumn(ColumnDefinition), // TODO distinction between ADD and ADD COLUMN
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateTableBody {
    ColumnsAndConstraints {
        columns: Vec<ColumnDefinition>,
        constraints: Option<Vec<NamedTableConstraint>>,
        without: Option<Name>,
    },
    AsSelect(Select),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnDefinition {
    pub col_name: Name,
    pub col_type: Option<Type>,
    pub constraints: Vec<NamedColumnConstraint>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedColumnConstraint {
    pub name: Option<Name>,
    pub constraint: ColumnConstraint,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ColumnConstraint {
    PrimaryKey {
        order: Option<SortOrder>,
        conflict_clause: Option<ResolveType>,
        auto_increment: bool,
    },
    NotNull {
        nullable: bool,
        conflict_clause: Option<ResolveType>,
    },
    Unique(Option<ResolveType>),
    Check(Expr),
    Default(DefaultValue),
    Collate { collation_name: String },
    ForeignKey(ForeignKeyClause),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedTableConstraint {
    pub name: Option<Name>,
    pub constraint: TableConstraint,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TableConstraint {
    PrimaryKey {
        columns: Vec<SortedColumn>,
        auto_increment: bool,
        conflict_clause: Option<ResolveType>,
    },
    Unique {
        columns: Vec<SortedColumn>,
        conflict_clause: Option<ResolveType>,
    },
    Check(Expr),
    ForeignKey {
        columns: Vec<IndexedColumn>,
        clause: ForeignKeyClause,
        deref_clause: DeferSubclause,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DefaultValue {
    Expr(Expr), // TODO
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForeignKeyClause {
    pub tbl_name: Name,
    pub columns: Option<Vec<IndexedColumn>>,
    pub args: Option<Vec<RefArg>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RefArg {
    OnDelete(RefAct),
    OnUpdate(RefAct),
    Match(Name),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RefAct {
    SetNull,
    SetDefault,
    Cascade,
    Restrict,
    NoAction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeferSubclause {
    pub deferrable: bool,
    pub init_deferred: Option<InitDeferredPred>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InitDeferredPred {
    InitiallyDeferred,
    InitiallyImmediate,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedColumn {
    pub col_name: Name,
    pub collation_name: Option<String>,
    pub order: Option<SortOrder>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Indexed {
    // idx name
    IndexedBy(Name),
    NotIndexed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SortedColumn {
    pub expr: Expr,
    pub order: Option<SortOrder>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Limit {
    pub expr: Expr,
    pub offset: Option<Expr>, // TODO distinction between LIMIT offset, count and LIMIT count OFFSET offset
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InsertBody {
    Select(Select),
    DefaultValues,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Set {
    pub col_name: Name,
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PragmaBody {
    Equals(PragmaValue),
    Call(PragmaValue),
}

pub type PragmaValue = String; // TODO

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TriggerTime {
    Before,
    After,
    InsteadOf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TriggerEvent {
    Delete,
    Insert,
    Update,
    // col names
    UpdateOf(Vec<Name>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TriggerCmd {
    Update {
        or_conflict: Option<ResolveType>,
        tbl_name: Name,
        sets: Vec<Set>,
        where_clause: Option<Expr>,
    },
    Insert {
        tbl_name: Name,
        col_names: Option<Vec<Name>>,
        select: Select,
    },
    Delete {
        tbl_name: Name,
        where_clause: Option<Expr>,
    },
    Select(Select),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResolveType {
    Rollback,
    Abort,
    Fail,
    Ignore,
    Replace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct With {
    pub recursive: bool,
    pub ctes: Vec<CommonTableExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommonTableExpr {
    pub tbl_name: Name,
    pub columns: Option<Vec<IndexedColumn>>,
    pub select: Select,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type {
    pub name: String, // TODO Validate
    pub size: Option<TypeSize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeSize {
    MaxSize(String),
    TypeSize(String, String),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    Deferred,
    Immediate,
    Exclusive,
}