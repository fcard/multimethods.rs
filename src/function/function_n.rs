use crate::function::helper_macros::*;

// Types

// -- Value returning functions

pub enum Function1 {
  S(fnbox_static!(a)),
  R(fnbox_ref!(a)),
}

pub enum Function2 {
  S(fnbox_static!(a,b)),
  R(fnbox_ref!(a,b)),
}

pub enum Function3 {
  S(fnbox_static!(a,b,c)),
  R(fnbox_ref!(a,b,c)),
}

pub enum Function4 {
  S(fnbox_static!(a,b,c,d)),
  R(fnbox_ref!(a,b,c,d)),
}

pub enum Function5 {
  S(fnbox_static!(a,b,c,d,e)),
  R(fnbox_ref!(a,b,c,d,e)),
}

pub enum Function6 {
  S(fnbox_static!(a,b,c,d,e,f)),
  R(fnbox_ref!(a,b,c,d,e,f)),
}

pub enum Function7 {
  S(fnbox_static!(a,b,c,d,e,f,g)),
  R(fnbox_ref!(a,b,c,d,e,f,g)),
}

pub enum Function8 {
  S(fnbox_static!(a,b,c,d,e,f,g,h)),
  R(fnbox_ref!(a,b,c,d,e,f,g,h)),
}

pub enum Function9 {
  S(fnbox_static!(a,b,c,d,e,f,g,h,i)),
  R(fnbox_ref!(a,b,c,d,e,f,g,h,i)),
}

pub enum Function10 {
  S(fnbox_static!(a,b,c,d,e,f,g,h,i,j)),
  R(fnbox_ref!(a,b,c,d,e,f,g,h,i,j)),
}

pub enum Function11 {
  S(fnbox_static!(a,b,c,d,e,f,g,h,i,j,k)),
  R(fnbox_ref!(a,b,c,d,e,f,g,h,i,j,k)),
}

pub enum Function12 {
  S(fnbox_static!(a,b,c,d,e,f,g,h,i,j,k,l)),
  R(fnbox_ref!(a,b,c,d,e,f,g,h,i,j,k,l)),
}

pub enum Function13 {
  S(fnbox_static!(a,b,c,d,e,f,g,h,i,j,k,l,m)),
  R(fnbox_ref!(a,b,c,d,e,f,g,h,i,j,k,l,m)),
}

pub enum Function14 {
  S(fnbox_static!(a,b,c,d,e,f,g,h,i,j,k,l,m,n)),
  R(fnbox_ref!(a,b,c,d,e,f,g,h,i,j,k,l,m,n)),
}

pub enum Function15 {
  S(fnbox_static!(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o)),
  R(fnbox_ref!(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o)),
}

pub enum Function16 {
  S(fnbox_static!(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p)),
  R(fnbox_ref!(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p)),
}

// -- Reference returning functions

pub struct Function1R {
  pub inner: fnbox_ref_return!(a)
}

pub struct Function2R {
  pub inner: fnbox_ref_return!(a,b)
}

pub struct Function3R {
  pub inner: fnbox_ref_return!(a,b,c)
}

pub struct Function4R {
  pub inner: fnbox_ref_return!(a,b,c,d)
}

pub struct Function5R {
  pub inner: fnbox_ref_return!(a,b,c,d,e)
}

pub struct Function6R {
  pub inner: fnbox_ref_return!(a,b,c,d,e,f)
}

pub struct Function7R {
  pub inner: fnbox_ref_return!(a,b,c,d,e,f,g)
}

pub struct Function8R {
  pub inner: fnbox_ref_return!(a,b,c,d,e,f,g,h)
}

pub struct Function9R {
  pub inner: fnbox_ref_return!(a,b,c,d,e,f,g,h,i)
}

pub struct Function10R {
  pub inner: fnbox_ref_return!(a,b,c,d,e,f,g,h,i,j)
}

pub struct Function11R {
  pub inner: fnbox_ref_return!(a,b,c,d,e,f,g,h,i,j,k)
}

pub struct Function12R {
  pub inner: fnbox_ref_return!(a,b,c,d,e,f,g,h,i,j,k,l)
}

pub struct Function13R {
  pub inner: fnbox_ref_return!(a,b,c,d,e,f,g,h,i,j,k,l,m)
}

pub struct Function14R {
  pub inner: fnbox_ref_return!(a,b,c,d,e,f,g,h,i,j,k,l,m,n)
}

pub struct Function15R {
  pub inner: fnbox_ref_return!(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o)
}

pub struct Function16R {
  pub inner: fnbox_ref_return!(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p)
}


// Impls

// -- Value returning functions

impl_function! {
  Function1(a:A)
}

impl_function! {
  Function2(a:A, b:B)
}

impl_function! {
  Function3(a:A, b:B, c:C)
}

impl_function! {
  Function4(a:A, b:B, c:C, d:D)
}

impl_function! {
  Function5(a:A, b:B, c:C, d:D, e:E)
}

impl_function! {
  Function6(a:A, b:B, c:C, d:D, e:E, f:F)
}

impl_function! {
  Function7(a:A, b:B, c:C, d:D, e:E, f:F, g:G)
}

impl_function! {
  Function8(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H)
}

impl_function! {
  Function9(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I)
}

impl_function! {
  Function10(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J)
}

impl_function! {
  Function11(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K)
}

impl_function! {
  Function12(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L)
}

impl_function! {
  Function13(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L, m:M)
}

impl_function! {
  Function14(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L, m:M, n:N)
}

impl_function! {
  Function15(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L, m:M, n:N, o:O)
}

impl_function! {
  Function16(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L, m:M, n:N, o:O, p:P)
}

// -- Reference returning functions

impl_ref_function! {
  Function1R(a:A)
}

impl_ref_function! {
  Function2R(a:A, b:B)
}

impl_ref_function! {
  Function3R(a:A, b:B, c:C)
}

impl_ref_function! {
  Function4R(a:A, b:B, c:C, d:D)
}

impl_ref_function! {
  Function5R(a:A, b:B, c:C, d:D, e:E)
}

impl_ref_function! {
  Function6R(a:A, b:B, c:C, d:D, e:E, f:F)
}

impl_ref_function! {
  Function7R(a:A, b:B, c:C, d:D, e:E, f:F, g:G)
}

impl_ref_function! {
  Function8R(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H)
}

impl_ref_function! {
  Function9R(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I)
}

impl_ref_function! {
  Function10R(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J)
}

impl_ref_function! {
  Function11R(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K)
}

impl_ref_function! {
  Function12R(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L)
}

impl_ref_function! {
  Function13R(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L, m:M)
}

impl_ref_function! {
  Function14R(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L, m:M, n:N)
}

impl_ref_function! {
  Function15R(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L, m:M, n:N, o:O)
}

impl_ref_function! {
  Function16R(a:A, b:B, c:C, d:D, e:E, f:F, g:G, h:H, i:I, j:J, k:K, l:L, m:M, n:N, o:O, p:P)
}

