// Derive Debug allows for Pretty Printing for Debugging
// Derive Clone allows implements a Clone method

// Used in boolean expressions... there's no reason to have both this
// and FuncOperand, but both are here for legacy reasons!!
#[derive(Debug, Clone)]
pub struct Operand {
    // Type of the operand: FLOAT, INT, STRING...
    pub code: usize,
    // Actual operand
    pub value: String,
}

impl Operand {
    pub fn new(code: usize, value: String) -> Operand {
        Operand { code, value }
    }
}

#[derive(Debug, Clone)]
pub struct ComparisonOp {
    // Corresponds to one of the codes describing what type
    // of literal value we have in this node: LESS_THAN, EQUALS...
    pub code: usize,
    // The operands
    pub left: Operand,
    pub right: Operand,
}

impl ComparisonOp {
    pub fn new(code: usize, left: Operand, right: Operand) -> ComparisonOp {
        ComparisonOp { code, left, right }
    }
}

//  For self-referential values, Box Pointer is required. They are dereferenced to get the value in them
#[derive(Debug, Clone)]
pub struct OrList {
    // Comparison to the left of the OR
    pub left: ComparisonOp,
    // OrList to the right of the OR; again,
    // This might be NULL if the right is a simple comparison
    pub right_or: Option<Box<OrList>>,
}

impl OrList {
    pub fn new(left: ComparisonOp, right_or: Option<Box<OrList>>) -> OrList {
        OrList { left, right_or }
    }
}

#[derive(Debug, Clone)]
pub struct AndList {
    pub left: Box<OrList>,
    // AndList to the right of the AND
    // This can be NULL if the right is a disjunction
    pub right_and: Option<Box<AndList>>,
}

impl AndList {
    pub fn new(left: Box<OrList>, right_and: Option<Box<AndList>>) -> AndList {
        AndList { left, right_and }
    }
}

#[derive(Debug, Clone)]
pub struct FuncOperand {
    pub code: usize,
    pub value: String,
}

impl FuncOperand {
    pub fn new(code: usize, value: String) -> FuncOperand {
        FuncOperand { code, value }
    }
}

#[derive(Debug, Clone)]
pub struct FuncOperator {
    pub code: usize,
    pub left_operator: Option<Box<FuncOperator>>,
    pub left_operand: Option<Box<FuncOperand>>,
    pub right: Option<Box<FuncOperator>>,
}

impl FuncOperator {
    pub fn new(
        code: usize,
        left_operator: Option<Box<FuncOperator>>,
        left_operand: Option<Box<FuncOperand>>,
        right: Option<Box<FuncOperator>>,
    ) -> FuncOperator {
        FuncOperator {
            code,
            left_operator,
            left_operand,
            right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableList {
    // Original Table Name
    pub table_name: String,

    // Alias of the table
    pub alias_as: String,

    // Next alias
    pub next: Option<Box<TableList>>,
}

impl TableList {
    pub fn new(table_name: String, alias_as: String, next: Option<Box<TableList>>) -> TableList {
        TableList {
            table_name,
            alias_as,
            next,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NameList {
    // Name
    pub name: String,

    // Next in list
    pub next: Option<Box<NameList>>,
}

impl NameList {
    pub fn new(name: String, next: Option<Box<NameList>>) -> NameList {
        NameList { name, next }
    }
}

// This holds the final structure for the SQL parser
#[derive(Debug, Clone)]
pub struct Final {
    final_function: FuncOperator, // the aggregate function (NULL if no agg)
    tables: TableList,            // the list of tables and aliases in the query
    and_list: bool,               // the predicate in the WHERE clause
    grouping_atts: NameList,      // grouping atts (NULL if no grouping)
    atts_to_select: NameList,     // the set of attributes in the SELECT (NULL if no such atts)
    distinct_atts: i64,           // 1 if there is a DISTINCT in a non-aggregate query
    distinct_func: i64,           // 1 if there is a DISTINCT in an aggregate query
}

impl Final {
    pub fn new(
        final_function: FuncOperator,
        tables: TableList,
        and_list: bool,
        grouping_atts: NameList,
        atts_to_select: NameList,
        distinct_atts: i64,
        distinct_func: i64,
    ) -> Final {
        Final {
            final_function,
            tables,
            and_list,
            grouping_atts,
            atts_to_select,
            distinct_atts,
            distinct_func,
        }
    }
}

// Global Values used by Comparison
pub const LESS_THAN: usize = 1;
pub const GREATER_THAN: usize = 2;
pub const EQUALS: usize = 3;

pub const DOUBLE: usize = 1;
pub const INT: usize = 2;
pub const STRING: usize = 3;
pub const NAME: usize = 4;

pub const ADD: usize = 1;
pub const SUB: usize = 2;
pub const MUL: usize = 3;
pub const DIV: usize = 4;
