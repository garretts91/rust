dnl  mpn_mul_basecase - from x86_64/coreisbr directory for fat binary.
dnl  Generated by configure - DO NOT EDIT.

define(OPERATION_mul_basecase)
define(__gmpn_mul_basecase, __gmpn_mul_basecase_coreisbr)
define(__gmpn_mul_basecasec,__gmpn_mul_basecasec_coreisbr)
define(__gmpn_preinv_mul_basecase,__gmpn_preinv_mul_basecase_coreisbr)
define(__gmpn_mul_basecase_cps,__gmpn_mul_basecase_cps_coreisbr)

dnl  For k6 and k7 gcd_1 calling their corresponding mpn_modexact_1_odd
ifdef(`__gmpn_modexact_1_odd',,
`define(__gmpn_modexact_1_odd,__gmpn_modexact_1_odd_coreisbr)')

define(MUL_TOOM22_THRESHOLD,20)
define(MUL_TOOM33_THRESHOLD,65)
define(SQR_TOOM2_THRESHOLD,28)
define(SQR_TOOM3_THRESHOLD,93)
define(BMOD_1_TO_MOD_1_THRESHOLD,30)

include(../../gmp-src/mpn/x86_64/coreisbr/mul_basecase.asm)

