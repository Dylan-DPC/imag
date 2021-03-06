# Conventions, best practices

This section goes about best practices in the imag codebase. It is mainly
focused on developers, but a user may read it for getting to know how imag
works.

Lets work our way up from the store and how to extend it to the commandline user
interface.

## Versioning

All imag crates are versioned with the same version number until we reach some
`"1.0.0"` version.
This means that all imag tools are only tested for compatibility with libraries
and such if their version numbers match.
It might not be possible to import one imag library in version 0.3.0 and another
one in 0.4.0 and make them work together.
It also means that if new tools are introduced into the imag codebase, they
might start with their first version not at 0.1.0 but at something like 0.5.0.

## Store and Entry functionality

A `Entry` does not offer much functionality by itself. So its the job of
libraries to _extend_ its functionality. This should never be done by wrapping
the `Entry` type itself but by providing and implementing an extension trait on
it.

Same goes for extending the `Store` type: never wrap it, always provide an
extension trait for it.

These two rules ensure that the type does not lose any functionality from a
wrapping. `Deref` could do that, but not over muliple levels, so extension
traits it is. It also most likely results in functions inside the extension
trait which all return a `Result<_, _>`.

## Libraries

In the next few sections, conventions and best practices for writing a imag
library are written down.

A developer of imag should read this carefully, a user may skip this section or
cross-read it for better understanding of the imag project.

### Library naming

Libraries which provide functionality for entries or the store (most likely
entries or both) should be named "libimagentrything" whereas "thing" stands for
what the library provides.

All other libraries should be prefixed with "libimag" at least. Most likely, one
will not write such a library but rather a "libimagentrything" library.

### Library scope

A library should never introduce utility functionality which could be useful for
other libraries as well. If there is no such functionality available, the
"libimagutil" might be a place where such a function would be put, or, if not
yet available, a "libimagentryutil" could be created.

If a library has to introduce free functions in its public interface, one should
think hard whether this is really necessary.

### Library error types/kinds

Libraries must use the "libimagerror" tools to create error types and kinds.
Most likely, a library needs some kinds for wrapping the errors from underlying
libraries, such as the store itself.

A library must _never_ introduce multiple error types, but is free to introduce
as many error kinds as required. Indeed, more kinds is better than fewer.

### Libraries with commandline frontends

Libraries with commandline frontends provide end-user functionality. Normally,
they depend on one or more "libimagentrything" libraries. They should be named
"libimagthing", though. For example: "libimagdiary", "libimagtimetrack" or
"libimagwiki", whereas the commandline frontends would be "imag-diary",
"imag-timetrack" and "imag-wiki", respectively.

If such a library needs to depend on another "libimagthing", for example if
"libimagdiary" needs to depend on "libimagnote", one should think about this and
whether the functionality could be outsourced to a more general
"libimagentrything". It is not forbidden, though.

A library which implements a functionality for imag may contain helper functions
for commandline stuff, but that is discouraged.

### Library testing

All libraries should be tested as much as possible. Sometimes it may not be
possible without a lot of effort, but still: more tests = better!

## Commandline tools

The next few sections describe how the commandline frontends are implemented.
Each imag functionality (or module) has its own library and a commandline
frontend for it.

The commandline frontends do contain little to no functionality. They simply
translate the commandline parameters and options to calls to the appropriate
library functions.

## Commandline tool testing

## Commandline interface

