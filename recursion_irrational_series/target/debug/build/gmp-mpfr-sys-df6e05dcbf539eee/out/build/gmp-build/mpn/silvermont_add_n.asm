dnl  mpn_add_n - from x86_64/silvermont directory for fat binary.
dnl  Generated by configure - DO NOT EDIT.

define(OPERATION_add_n)
define(__gmpn_add_n, __gmpn_add_n_silvermont)
define(__gmpn_add_nc,__gmpn_add_nc_silvermont)
define(__gmpn_preinv_add_n,__gmpn_preinv_add_n_silvermont)
define(__gmpn_add_n_cps,__gmpn_add_n_cps_silvermont)

dnl  For k6 and k7 gcd_1 calling their corresponding mpn_modexact_1_odd
ifdef(`__gmpn_modexact_1_odd',,
`define(__gmpn_modexact_1_odd,__gmpn_modexact_1_odd_silvermont)')

define(MUL_TOOM22_THRESHOLD,19)
define(MUL_TOOM33_THRESHOLD,66)
define(SQR_TOOM2_THRESHOLD,24)
define(SQR_TOOM3_THRESHOLD,97)

include(../../gmp-src/mpn/x86_64/silvermont/aors_n.asm)

