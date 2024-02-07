dnl  mpn_addmul_2 - from x86_64/bd1 directory for fat binary.
dnl  Generated by configure - DO NOT EDIT.

define(OPERATION_addmul_2)
define(__gmpn_addmul_2, __gmpn_addmul_2_bd1)
define(__gmpn_addmul_2c,__gmpn_addmul_2c_bd1)
define(__gmpn_preinv_addmul_2,__gmpn_preinv_addmul_2_bd1)
define(__gmpn_addmul_2_cps,__gmpn_addmul_2_cps_bd1)

dnl  For k6 and k7 gcd_1 calling their corresponding mpn_modexact_1_odd
ifdef(`__gmpn_modexact_1_odd',,
`define(__gmpn_modexact_1_odd,__gmpn_modexact_1_odd_bd1)')

define(MUL_TOOM22_THRESHOLD,20)
define(MUL_TOOM33_THRESHOLD,57)
define(SQR_TOOM2_THRESHOLD,24)
define(SQR_TOOM3_THRESHOLD,85)
define(BMOD_1_TO_MOD_1_THRESHOLD,27)

include(../../gmp-src/mpn/x86_64/bd1/addmul_2.asm)

