set hlsearch
set incsearch
set laststatus=2
set nf=alpha,hex
set number
set paste
set relativenumber
set ruler
set showcmd

" Expand a tab into 4 spaces when editing rust source file.
filetype plugin on
filetype indent on
autocmd FileType rust setlocal expandtab shiftwidth=4 softtabstop=4 tabstop=4

