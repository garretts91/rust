dnl  mpn_sqr_basecase - from x86_64/silvermont directory for fat binary.
dnl  Generated by configure - DO NOT EDIT.

define(OPERATION_sqr_basecase)
define(__gmpn_sqr_basecase, __gmpn_sqr_basecase_silvermont)
define(__gmpn_sqr_basecasec,__gmpn_sqr_basecasec_silvermont)
define(__gmpn_preinv_sqr_basecase,__gmpn_preinv_sqr_basecase_silvermont)
define(__gmpn_sqr_basecase_cps,__gmpn_sqr_basecase_cps_silvermont)

dnl  For k6 and k7 gcd_1 calling their corresponding mpn_modexact_1_odd
ifdef(`__gmpn_modexact_1_odd',,
`define(__gmpn_modexact_1_odd,__gmpn_modexact_1_odd_silvermont)')

define(MUL_TOOM22_THRESHOLD,19)
define(MUL_TOOM33_THRESHOLD,66)
define(SQR_TOOM2_THRESHOLD,24)
define(SQR_TOOM3_THRESHOLD,97)

include(../../gmp-src/mpn/x86_64/silvermont/sqr_basecase.asm)

