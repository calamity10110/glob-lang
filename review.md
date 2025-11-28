# review full project rename the language GLOB,

# add more function:

## allow to do import scripts/packages, define definitions, add async/non-async functions, in block:

'''
import python : numpy.
import python : matplotlib.
import rust : tokio,
import blob_pak_1.
import blob_pak_2.
'''
is the same as
'''
import [
python : (numpy, matplotlib),
rust : tokio,
blob_pak_1,
blob_pak_2].
'''
or
'''
import python : {numpy, matplotlib}.
import rust : tokio.
import (blob_pak_1, blob_pak_2).
'''

## [], {}, () work the same, as long as they match.

## ? is same as mut (mutable), allow to change the value.

## global variables are manage by async-func, static variables are manage by func(non-async) and async-func.

## add @asy, @fn, @cs, @?var, @var, @lst, @map, @set, @str, @num, @int, @float, @bool, @char, @null, @any, @void, @unit, @true, @false, @true?, @false?, @true!, @false!, @true!!, @false!!, @global, @local, @var, @mut, @ref, @own, @copy, @move, @drop, @drop!, @drop!!, @?var, @?lst, @?map, @?set, @?str, @?num, @?int, @?float, @?bool, @?char, @?null, @?any, @?void, @?unit, @?true, @?false, @?true?, @?false?, @?true!, @?false!, @?true!!, @?false!!, @?ref, @?own, @?copy, @?move, @?drop, @?drop!, @?drop!!

## add @less, @more, @equal, @not_equal, @less_equal, @more_equal, @and, @or, @not, @xor, @impl, @if, @elif, @else, @for, @while, @loop, @break, @continue, @return, @sort, @reverse, @len, @max, @min, @sum, @avg, @mean, @median, @mode, @variance, @stddev, @covariance, @correlation, @rank, @percentile, @quantile, @entropy, @log, @ln, @exp, @sqrt, @cbrt, @abs, @floor, @ceil, @round, @trunc, @frac, @sign, @signum, @sgn, @sgn!, @sgn!!, @?less, @?more, @?equal, @?not_equal, @?less_equal, @?more_equal, @?and, @?or, @?not, @?xor, @?impl, @?if, @?elif, @?else, @?for, @?while, @?loop, @?break, @?continue, @?return, @?sort, @?reverse, @?len, @?max, @?min, @?sum, @?avg, @?mean, @?median, @?mode, @?variance, @?stddev, @?covariance, @?correlation, @?rank, @?percentile, @?quantile, @?entropy, @?log, @?ln, @?exp, @?sqrt, @?cbrt, @?abs, @?floor, @?ceil, @?round, @?trunc, @?frac, @?sign, @?signum, @?sgn, @?sgn!, @?sgn!!

# test, debug all features in documentation.

# add information to syntax.md and structure.md to fully explain the language syntax & structure, coding patterns, best practice, examples, add annotation to codebase, aim for beginner.

# review and finish update documentation: compiler, intergration, plan, progress, test_verification_report, tui, webui, syntax, structure(project and language) readme, rebrand_summary.
