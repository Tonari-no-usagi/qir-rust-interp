; 回転ゲート(Rx)のテスト
; |0> 状態を PI/2 回転させて |+> 状態 (またはその近傍) にする

define void @rotate_test() {
  ; Rx(PI/2, q0)
  ; PI/2 = 1.57079632679
  call void @__quantum__qis__rx__body(double 1.57079632679, i64 0)
  
  ; 結果を表示するために測定（確率は 50/50 になるはず）
  call void @__quantum__qis__mz__body(i64 0, i64 100)
  ret void
}
