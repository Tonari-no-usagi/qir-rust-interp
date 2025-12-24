; 量子テレポーテーションのサンプル
; 3つの量子ビットを使用: 0(Alice's q), 1(Alice's ent), 2(Bob's ent)

define void @teleport() {
  ; 1. ベル状態の準備 (q1, q2)
  call void @__quantum__qis__h__body(i64 1)
  call void @__quantum__qis__cnot__body(i64 1, i64 2)

  ; 2. Aliceが送りたい量子状態の作成 (q0)
  call void @__quantum__qis__x__body(i64 0) ; |1> 状態を作成

  ; 3. Aliceの操作
  call void @__quantum__qis__cnot__body(i64 0, i64 1)
  call void @__quantum__qis__h__body(i64 0)

  ; 4. Aliceの測定
  call void @__quantum__qis__mz__body(i64 0, i64 100) ; q0の結果を addr 100 に保存
  call void @__quantum__qis__mz__body(i64 1, i64 101) ; q1の結果を addr 101 に保存

  ; 5. 古典制御によるBobの補正
  ; Aliceのq1の測定結果が1なら、BobはXゲートを適用
  %r1 = call i1 @__quantum__qis__read_result__body(i64 101)
  br i1 %r1, label %apply_x, label %check_z

apply_x:
  call void @__quantum__qis__x__body(i64 2)
  br label %check_z

check_z:
  ; Aliceのq0の測定結果が1なら、BobはZゲートを適用
  %r0 = call i1 @__quantum__qis__read_result__body(i64 100)
  br i1 %r0, label %apply_z, label %finish

apply_z:
  call void @__quantum__qis__z__body(i64 2)
  br label %finish

finish:
  ret void
}
