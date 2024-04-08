use super::models::{AExpr, BinaryOperation};
use crate::mathematical_functions::power::power;

impl AExpr {
    pub fn evaluate(&self) -> i64 {
        match self {
            AExpr::Num(stevilo) => *stevilo,
            AExpr::Variable(neznanka) => panic!("Ne bi smelo bit neznank"),
            AExpr::BinOp(levo, operacija, desno) => match operacija {
                BinaryOperation::Add => levo.evaluate() + desno.evaluate(),
                BinaryOperation::Sub => levo.evaluate() - desno.evaluate(),
                BinaryOperation::Mul => levo.evaluate() * desno.evaluate(),
                BinaryOperation::Pow => power(levo.evaluate(),desno.evaluate())
            }
        }
    }

    pub fn evaluate_map(
        &self,
        vars: &std::collections::HashMap<String, Option<i64>>,
    ) -> Option<i64> {
        panic!("Implement variable evaluation")
    }
}
