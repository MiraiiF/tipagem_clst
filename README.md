# Programming Languages project
My project for the Programming Languages I course in Universidade Federal do Ceará (UFC).

# Defining my custom language
The language being used here is a variation of the simply typed lambda calculus.

# Typing
The types of the system are:

Nat - A natural number

Bool - A boolean value (either true or false)

( type1 -> type2 ) - a function of input type1 and output type2

A hyphen (-) implies that the types don't match.

An exclamation mark (!) implies that there is an error with the sentence structure.

## Grammar
Every word is separated by blank spaces (" ").

An conditional statement can be described like:
if BooleanValue then Statement1 else Statement2 endif
As it is a functional language, the else statement must be specified. Don't forget the endif part, too. Both statements have to share the same type! 

A function can be described like:

lambda VariableName : VariableType . Statement end

The function will be of type ( VariableType -> StatementType )

You can also apply that function to a value (the values have to match or the output will be a hyphen, as said before):

( function value )

The type output here will be StatementType, judging by the previous function definition.

There are also three functions already present in the language:

suc - successor of a number ( Nat -> Nat )

pred - predecessor of a number ( Nat -> Nat )

ehzero - is the number zero? ( Nat -> Bool )

## What's the point?
This is just a type inference program for that language. It doesn't compute the final result, just its type.

## Examples
suc 1

Nat


lambda f : Bool . if f then 1 else 2 endif end

( Bool -> Nat )


lambda banana : ( Nat -> Bool ) . lambda peel : Nat . if ( banana peel ) then 100 else 0 endif end end

( ( Nat -> Bool ) -> ( Nat -> Nat ) )
