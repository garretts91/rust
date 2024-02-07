dnl  mpn_cnd_add_n - from x86_64/atom directory for fat binary.
dnl  Generated by configure - DO NOT EDIT.

define(OPERATION_cnd_add_n)
define(__gmpn_cnd_add_n, __gmpn_cnd_add_n_atom)
define(__gmpn_cnd_add_nc,__gmpn_cnd_add_nc_atom)
define(__gmpn_preinv_cnd_add_n,__gmpn_preinv_cnd_add_n_atom)
define(__gmpn_cnd_add_n_cps,__gmpn_cnd_add_n_cps_atom)

dnl  For k6 and k7 gcd_1 calling their corresponding mpn_modexact_1_odd
ifdef(`__gmpn_modexact_1_odd',,
`define(__gmpn_modexact_1_odd,__gmpn_modexact_1_odd_atom)')

define(MUL_TOOM22_THRESHOLD,12)
define(MUL_TOOM33_THRESHOLD,74)
define(SQR_TOOM2_THRESHOLD,22)
define(SQR_TOOM3_THRESHOLD,73)
define(BMOD_1_TO_MOD_1_THRESHOLD,16)

include(../../gmp-src/mpn/x86_64/atom/cnd_add_n.asm)

