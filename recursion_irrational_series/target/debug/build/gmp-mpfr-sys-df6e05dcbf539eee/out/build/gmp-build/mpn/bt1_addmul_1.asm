dnl  mpn_addmul_1 - from x86_64/bt1 directory for fat binary.
dnl  Generated by configure - DO NOT EDIT.

define(OPERATION_addmul_1)
define(__gmpn_addmul_1, __gmpn_addmul_1_bt1)
define(__gmpn_addmul_1c,__gmpn_addmul_1c_bt1)
define(__gmpn_preinv_addmul_1,__gmpn_preinv_addmul_1_bt1)
define(__gmpn_addmul_1_cps,__gmpn_addmul_1_cps_bt1)

dnl  For k6 and k7 gcd_1 calling their corresponding mpn_modexact_1_odd
ifdef(`__gmpn_modexact_1_odd',,
`define(__gmpn_modexact_1_odd,__gmpn_modexact_1_odd_bt1)')

define(MUL_TOOM22_THRESHOLD,24)
define(MUL_TOOM33_THRESHOLD,66)
define(SQR_TOOM2_THRESHOLD,30)
define(SQR_TOOM3_THRESHOLD,101)
define(BMOD_1_TO_MOD_1_THRESHOLD,20)

include(../../gmp-src/mpn/x86_64/bt1/aorsmul_1.asm)

