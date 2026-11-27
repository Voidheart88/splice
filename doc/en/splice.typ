// Splice Documentation - English
// Main document file

#import "@preview/lettre:0.2.2": lettre, letter
#import "@preview/fletcher:0.4.0": fletcher
#import "@preview/touille:0.1.0: touille, outline-entry

#show: touille(
  title: [Splice Documentation],
  author: [Splice Development Team],
  date: datetime.today(),
)

= Splice - A Blazingly Fast Circuit Simulator

This documentation covers the architecture, design decisions, and developer guidelines for the Splice circuit simulator.

#include "chapters/intro.typ"
#include "chapters/user_manual.typ"
#include "chapters/architecture.typ"
