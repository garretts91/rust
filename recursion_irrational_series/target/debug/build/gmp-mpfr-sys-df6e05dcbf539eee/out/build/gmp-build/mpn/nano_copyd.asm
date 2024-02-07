dnl  mpn_copyd - from x86_64/nano directory for fat binary.
dnl  Generated by configure - DO NOT EDIT.

define(OPERATION_copyd)
define(__gmpn_copyd, __gmpn_copyd_nano)
define(__gmpn_copydc,__gmpn_copydc_nano)
define(__gmpn_preinv_copyd,__gmpn_preinv_copyd_nano)
define(__gmpn_copyd_cps,__gmpn_copyd_cps_nano)

dnl  For k6 and k7 gcd_1 calling their corresponding mpn_modexact_1_odd
ifdef(`__gmpn_modexact_1_odd',,
`define(__gmpn_modexact_1_odd,__gmpn_modexact_1_odd_nano)')

define(MUL_TOOM22_THRESHOLD,27)
define(MUL_TOOM33_THRESHOLD,38)
define(SQR_TOOM2_THRESHOLD,52)
define(SQR_TOOM3_THRESHOLD,73)
define(BMOD_1_TO_MOD_1_THRESHOLD,22)

include(../../gmp-src/mpn/x86_64/nano/copyd.asm)

