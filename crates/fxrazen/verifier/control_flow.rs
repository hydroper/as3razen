use crate::ns::*;

pub(crate) struct ControlFlowParent<'a> {
    pub parent: ControlFlowBlock,
    pub next_siblings: &'a [Rc<Directive>],
}

pub(crate) struct ControlFlowAnalyser;

impl ControlFlowAnalyser {
    pub fn analyse_directives<'a>(
        list: &[Rc<Directive>],
        cfg: &ControlFlowGraph,
        building_block: &mut Vec<Rc<Directive>>,
        ascending_parents: &[ControlFlowParent<'a>]
    ) {
        todo_here();
    }
}