use crate::ErrOr;
use seaso::{
    dynamics::{Denotation, Knowledge},
    statics::ExecutableError,
    DomainId, ExecutableConfig, PartName, Program,
};

pub enum WfErr<'a> {
    RepeatedlyDefinedPart(&'a PartName),
    ExecutableError(ExecutableError<'a>),
    DomainCycle(DomainId),
}

pub fn well_founded_denotation(program: &Program) -> Result<Denotation<Knowledge>, WfErr> {
    program.repeatedly_defined_part().err_or(()).map_err(WfErr::RepeatedlyDefinedPart)?;
    let ep = program.executable(ExecutableConfig::default()).map_err(WfErr::ExecutableError)?;
    ep.unbounded_domain_cycle().err_or(()).map_err(Clone::clone).map_err(WfErr::DomainCycle)?;
    Ok(ep.denotation().denotation)
}
