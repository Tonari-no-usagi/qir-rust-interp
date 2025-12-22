; Sample QIR for Bell State
define void @BellState() {
  call void @__quantum__qis__h__body(%Qubit* null)
  call void @__quantum__qis__cnot__body(%Qubit* null, %Qubit* inttoptr (i64 1 to %Qubit*))
  %res0 = call %Result* @__quantum__qis__mz__body(%Qubit* null)
  %res1 = call %Result* @__quantum__qis__mz__body(%Qubit* inttoptr (i64 1 to %Qubit*))
  ret void
}
