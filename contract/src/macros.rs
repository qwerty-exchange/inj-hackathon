#[macro_export]
macro_rules! ensure_state {
    ($current: expr, $expected: expr) => {
        if $current != $expected {
            return Err(ContractError::WrongPropositionStatus {
                expected: $expected,
                current: $current,
            });
        }
    };
}
