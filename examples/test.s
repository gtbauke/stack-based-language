fun println do
  "\n" + print
end

fun main do
  0 while dup 10 < do
    dup 2 % 0 == if
      dup print " is even" println
    else
      dup print " is odd" println
    end

    1 +
  end

  drop
end
