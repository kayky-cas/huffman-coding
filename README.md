# Codificação

-   Exemplo de valores em um Arquivo para os exemplos a seguir:

```
a -> 1%
b -> 5%
c -> 9%
d -> 20%
e -> 3%
f -> 2%
g -> 6%
h -> 12%
i -> 9%
j -> 15%
k -> 10%
l -> 8%
```

#### Forma 1

```
byte 1    = [0][0][0][0] [0][1][0][1]
byte 2    = [0][0][0][0] [1][0][1][0]
Novo byte = [0][1][0][1] [1][0][1][0] (byte1 << 4
| byte 2)
```

#### Forma 2

```
d -> 20% = 1
j -> 15% = 10
h -> 12% = 100
k -> 10% = 1000
c -> 9%  = 10000
i -> 9%  = 100000
l -> 8%  = 1000000
g -> 6%  = 10000000
b -> 5%  = 100000000
e -> 3%  = 1000000000
f -> 2%  = 10000000000
a -> 1%  = 100000000000
```

#### Forma 3 (Codificação de Huffman)

-   Criar 12 (números de variações) árvores binárias.
-   Junte as que tem um número mais baixo um com o outro

```
            ...
           /
        (15)
       /    \
     (6)     (l)
    /   \
  (3)   (e)
 /   \
(f) (a)
```

-   Um dos dois nodos vale 0, e ou outro 1. Então se continuassemos a árvore, os valores seriam:

```
a -> 1000001
c -> 1001
...
d -> 00
```
