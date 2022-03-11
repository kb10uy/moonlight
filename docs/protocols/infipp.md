# Protocol - Infipp
Stands for "**IN**terleaved **FI**xed **P**oint **P**acking"

## 概要
* あらかじめ送受信側で策定した幅の固定小数を扱うためのフォーマット。
* 1 ピクセルに対して 1 つの値を割り当てる。
* Linear 色空間を想定するため、途中に Gamma 空間が入る場合は各実装で個別に対応しなければならない。
