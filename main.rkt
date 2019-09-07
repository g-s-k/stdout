#!/usr/local/bin/racket
#lang racket/base

(define (print-hello n)
  (if (> n 0)
      (begin
        (writeln "hello world")
        (print-hello (- n 1)))
      (void)))

(print-hello 200)
