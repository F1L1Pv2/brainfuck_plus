#define macro 
//#define macro2
#ifdef macro
`"Y"`
#else
`"N"`
#endif
.`"\n"`.

#ifndef macro2
`"L"`
.`"\n"`.
#endif