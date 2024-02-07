dnl  mpn_gcd_11 - from x86_64/bt2 directory for fat binary.
dnl  Generated by configure - DO NOT EDIT.

define(OPERATION_gcd_11)
define(__gmpn_gcd_11, __gmpn_gcd_11_bt2)
define(__gmpn_gcd_1c1,__gmpn_gcd_1c1_bt2)
define(__gmpn_preinv_gcd_11,__gmpn_preinv_gcd_11_bt2)
define(__gmpn_gcd_11_cps,__gmpn_gcd_11_cps_bt2)

dnl  For k6 and k7 gcd_1 calling their corresponding mpn_modexact_1_odd
ifdef(`__gmpn_modexact_1_odd',,
`define(__gmpn_modexact_1_odd,__gmpn_modexact_1_odd_bt2)')

define(MUL_TOOM22_THRESHOLD,25)
define(MUL_TOOM33_THRESHOLD,32)
define(SQR_TOOM2_THRESHOLD,20)
define(SQR_TOOM3_THRESHOLD,97)
define(BMOD_1_TO_MOD_1_THRESHOLD,15)

include(../../gmp-src/mpn/x86_64/bt2/gcd_11.asm)

