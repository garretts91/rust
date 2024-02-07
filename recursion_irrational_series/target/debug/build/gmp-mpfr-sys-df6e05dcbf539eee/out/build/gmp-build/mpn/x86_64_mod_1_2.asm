dnl  mpn_mod_1s_2p - from x86_64 directory for fat binary.
dnl  Generated by configure - DO NOT EDIT.

define(OPERATION_mod_1_2)
define(__gmpn_mod_1s_2p, __gmpn_mod_1s_2p_x86_64)
define(__gmpn_mod_1cs_2p,__gmpn_mod_1cs_2p_x86_64)
define(__gmpn_preinv_mod_1s_2p,__gmpn_preinv_mod_1s_2p_x86_64)
define(__gmpn_mod_1s_2p_cps,__gmpn_mod_1s_2p_cps_x86_64)

dnl  For k6 and k7 gcd_1 calling their corresponding mpn_modexact_1_odd
ifdef(`__gmpn_modexact_1_odd',,
`define(__gmpn_modexact_1_odd,__gmpn_modexact_1_odd_x86_64)')

define(MUL_TOOM22_THRESHOLD,27)
define(MUL_TOOM33_THRESHOLD,81)
define(SQR_TOOM2_THRESHOLD,36)
define(SQR_TOOM3_THRESHOLD,117)
define(BMOD_1_TO_MOD_1_THRESHOLD,15)

include(../../gmp-src/mpn/x86_64/mod_1_2.asm)

