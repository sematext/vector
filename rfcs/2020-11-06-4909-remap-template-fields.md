# RFC 4909 - 2020-11-06 - Remap support for template strings

We would like to unify the templating syntax within configuration fields with
the Remap language.


## Scope

This RFC will look into ways we can use Remap whilst still supporting the current
method of templating fields.


## Motivation

The Loki sink wants a list of the fields that were used in the template. It
uses this list to remove these fields from the event sent to Loki. With Remap
this list becomes dynamic. So the script `if .foo { .bar } else { .baz }` 
means sometimes .bar is used and other times .baz is used.

## Internal Proposal

As there are a number of complications with working out which fields are used
in the output, we could only support this feature if the old style template
syntax is used. If Remap is used, the user won't have the option to remove these
fields from the message in the Loki sink.

(This is the simplest option.)

## Rationale

The benefits of using Remap are:

 - One familiar syntax and function reference for Vector.
 - Access to all of remap's functions for templating.
 - Less code to manage (once the old template fields are deprecated).


## Prior Art

[Handlebars JS](https://handlebarsjs.com/guide/#html-escaping) treats double
brackets (`{{..}}`) as one form of templating for most of it's language features,
but if you want special treatment (not escaping html) you can surround your
template fields with three brackets (`{{{..}}}`).


## Drawbacks

There could be an additional maintenance burden. Should the need to track fields
used in the script be implemented that is a fairly significant complication to
the script execution process that will need to always be kept in mind when 
future additions to the language are being implemented.


## Alternatives

### Do nothing

We do already have the existing template syntax. Perhaps we can stick with this.

The advantage of using Remap for these fields are that it allows more 
flexibility in defining how the event is used. However, given that remap can be 
used as a transform, should the user really need this they could put a Remap 
transform in the process to process these fields so they can be easily used in 
the template for the next phase.

### Returning fields at load time

To fix the issue with returning fields so that the Loki sink removes them, whilst
Remap is parsing the script, it could keep track of all fields that are being
used in the script.

This script, `if .foo { .bar } else { .baz }`, would result in all three
fields being returned - `.foo`, `.bar` and `.baz`, and subsequently removed
from the message sent to Loki.


### Returning fields at run time

Remap could keep track of all the fields that are read from as it runs and
return these fields. So the script, `if .foo { .bar } else { .baz }`,
would result in `.foo` and either `.bar` or `.baz` being returned.

If necessary, Remap could distinguish between fields that are used in the
condition and those used it the result, so only `.bar` or `.baz` could be
returned.

There are likely to be a number of edge cases that would need to be thought
through if we took this approach. For example, if a field is used in a function
but it's value is only used to influence the result - should it be included?

```
replace(.field1, .field2, .field3)
```

The result is the value of `field1`, but with any occurrence of the value of 
`field2` being replaced by `field3`. Which fields would be correct to include
in this list?


## Outstanding Questions

- How important is it for the Loki sink to remove fields used in the template
  from the event?

## Plan Of Attack

Incremental steps that execute this change. Generally this is in the form of:

- [ ] Submit a PR with spike-level code _roughly_ demonstrating the change.
- [ ] Incremental change #1
- [ ] Incremental change #2
- [ ] ...

Note: This can be filled out during the review process.