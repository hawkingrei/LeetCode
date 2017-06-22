def fib(n):#{
  if n<=2 :
    return 1;
  else:
    return fib(n-1)+fib(n-2);

if __name__ == '__main__':
  for x in range(10):
    print(fib(x))