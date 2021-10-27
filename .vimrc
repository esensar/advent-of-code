if exists('g:loaded_aoc_plugin')
  finish
endif
let g:loaded_aoc_plugin = 1

function! LoadAndEditProblem(year, day)
	execute "!scripts/load_problem " . a:year . " " . a:day
	execute "edit " . a:year . "/" . printf("%02d", a:day) . "/solution.*"
endfunction

function! LoadAndEditLatestProblem()
	call LoadAndEditProblem(strftime("%Y"), strftime("%d"))
endfunction

function! SubmitSolution(solution)
	let l:parts = split(expand("%"), "/")
	execute "!scripts/submit_solution " . l:parts[-3] . " " . l:parts[-2]
endfunction

command! -nargs=0 AOCLoadAndEditLatest :call LoadAndEditLatestProblem()
command! -nargs=* AOCLoadAndEdit :call LoadAndEditProblem(<f-args>)
command! -nargs=1 AOCSubmit :call SubmitSolution(<f-args>)
