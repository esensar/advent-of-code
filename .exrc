if exists('g:loaded_aoc_plugin')
  finish
endif
let g:loaded_aoc_plugin = 1

function! LoadAndEditProblem(year, day, ...)
  let fileextension = get(a:, 1, "*")
  execute "!scripts/load_problem " . a:year . " " . a:day
  execute "edit " . a:year . "/" . printf("%02d", a:day) . "/solution." . fileextension
endfunction

function! LoadAndEditLatestProblem(...)
  call LoadAndEditProblem(strftime("%Y"), strftime("%-d"), get(a:, 1, "*"))
endfunction

function! EditProblem(year, day, ...)
  let fileextension = get(a:, 1, "*")
  execute "edit " . a:year . "/" . printf("%02d", a:day) . "/solution." . fileextension
endfunction

function! EditLatestProblem(...)
  call EditProblem(strftime("%Y"), strftime("%-d"), get(a:, 1, "*"))
endfunction

function! SubmitSolution(solution)
  let l:parts = split(expand("%"), "/")
  execute "!scripts/submit_solution " . l:parts[-3] . " " . trim(l:parts[-2], "0", 1) . " " . a:solution
endfunction

command! -nargs=? AOCLoadAndEditLatest :call LoadAndEditLatestProblem(<f-args>)
command! -nargs=* AOCLoadAndEdit :call LoadAndEditProblem(<f-args>)
command! -nargs=? AOCEditLatest :call EditLatestProblem(<f-args>)
command! -nargs=* AOCEdit :call EditProblem(<f-args>)
command! -nargs=1 AOCSubmit :call SubmitSolution(<f-args>)
