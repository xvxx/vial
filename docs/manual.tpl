<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>vial: a micro micro-framework</title>
    <style type="text/css">
      body {
        margin: 40px auto;
        line-height: 1.6;
        font-size: 20px;
        color: #444;
        padding: 16px;
        font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Helvetica,
          Arial, sans-serif, Apple Color Emoji, Segoe UI Emoji;
      }

      /* big logo */
      h1:first-of-type {
        font-size: 80px;
        margin-bottom: 0;
        margin-top: -20px;
        padding-top: 0;
      }
      h4:first-of-type {
        margin: 0;
        font-size: 20px;
        color: #aaa;
        margin-top: -10px;
        margin-bottom: 20px;
        font-weight: normal;
        font-style: italic;
      }
      img:first-of-type {
        margin-top: -25px;
        margin-right: 20px;
      }

      /* structure */
      nav {
        float: left;
        padding-top: 0;
      }
      main {
        margin: 0 auto;
        max-width: 800px;
      }

      /* responsive */
      @media (max-width: 1500px) {
        main {
          padding-left: 75px;
          float: left;
        }
        img:first-of-type {
          margin-top: -45px;
        }
      }

      @media (max-width: 1180px) {
        nav {
          display: none;
        }
        main {
          float: left;
        }
      }

      /* nav  */
      nav h3 {
        margin: 0;
        padding-bottom: 0;
        padding-left: 40px;
        font-size: 22px;
      }
      nav h3:not(:first-of-type) {
        margin-top: 40px;
      }
      nav h3 + ul {
        margin-top: 10px;
      }
      nav ul {
        list-style-type: none;
      }
      nav ul ul {
        list-style-type: disc;
      }

      /* regular stuff */
      a {
        color: black;
        text-decoration: none;
        border-bottom: 1px solid black;
      }
      a:hover {
        border-bottom: none;
        color: fuchsia;
      }
      a:visited {
        color: #666;
        text-decoration: none;
      }
      h1 {
        font-size: 2em;
      }
      h2 {
        font-size: 1.75em;
      }
      h2,
      h3 {
        border-bottom: 1px solid #ddd;
      }
      h1,
      h2,
      h3 {
        line-height: 1.2;
      }
      p {
        text-align: justify;
      }

      /* pandoc styles */
      code {
        white-space: pre-wrap;
      }
      span.smallcaps {
        font-variant: small-caps;
      }
      span.underline {
        text-decoration: underline;
      }
      div.column {
        display: inline-block;
        vertical-align: top;
        width: 50%;
      }
      div.hanging-indent {
        margin-left: 1.5em;
        text-indent: -1.5em;
      }
      pre > code.sourceCode {
        white-space: pre;
        position: relative;
      }
      pre > code.sourceCode > span {
        display: inline-block;
        line-height: 1.25;
      }
      pre > code.sourceCode > span:empty {
        height: 1.2em;
      }
      code.sourceCode > span {
        color: inherit;
        text-decoration: inherit;
      }
      div.sourceCode {
        margin: 1em 0;
      }
      pre.sourceCode {
        margin: 0;
      }
      @media screen {
        div.sourceCode {
          overflow: auto;
        }
      }
      @media print {
        pre > code.sourceCode {
          white-space: pre-wrap;
        }
        pre > code.sourceCode > span {
          text-indent: -5em;
          padding-left: 5em;
        }
      }
      pre.numberSource code {
        counter-reset: source-line 0;
      }
      pre.numberSource code > span {
        position: relative;
        left: -4em;
        counter-increment: source-line;
      }
      pre.numberSource code > span > a:first-child::before {
        content: counter(source-line);
        position: relative;
        left: -1em;
        text-align: right;
        vertical-align: baseline;
        border: none;
        display: inline-block;
        -webkit-touch-callout: none;
        -webkit-user-select: none;
        -khtml-user-select: none;
        -moz-user-select: none;
        -ms-user-select: none;
        user-select: none;
        padding: 0 4px;
        width: 4em;
        color: #aaaaaa;
      }
      pre.numberSource {
        margin-left: 3em;
        border-left: 1px solid #aaaaaa;
        padding-left: 4px;
      }
      div.sourceCode {
      }
      @media screen {
        pre > code.sourceCode > span > a:first-child::before {
          text-decoration: underline;
        }
      }
      code span.al {
        color: #ff0000;
        font-weight: bold;
      } /* Alert */
      code span.an {
        color: #60a0b0;
        font-weight: bold;
        font-style: italic;
      } /* Annotation */
      code span.at {
        color: #7d9029;
      } /* Attribute */
      code span.bn {
        color: #40a070;
      } /* BaseN */
      code span.bu {
      } /* BuiltIn */
      code span.cf {
        color: #007020;
        font-weight: bold;
      } /* ControlFlow */
      code span.ch {
        color: #4070a0;
      } /* Char */
      code span.cn {
        color: #880000;
      } /* Constant */
      code span.co {
        color: #60a0b0;
        font-style: italic;
      } /* Comment */
      code span.cv {
        color: #60a0b0;
        font-weight: bold;
        font-style: italic;
      } /* CommentVar */
      code span.do {
        color: #ba2121;
        font-style: italic;
      } /* Documentation */
      code span.dt {
        color: #902000;
      } /* DataType */
      code span.dv {
        color: #40a070;
      } /* DecVal */
      code span.er {
        color: #ff0000;
        font-weight: bold;
      } /* Error */
      code span.ex {
      } /* Extension */
      code span.fl {
        color: #40a070;
      } /* Float */
      code span.fu {
        color: #06287e;
      } /* Function */
      code span.im {
      } /* Import */
      code span.in {
        color: #60a0b0;
        font-weight: bold;
        font-style: italic;
      } /* Information */
      code span.kw {
        color: #007020;
        font-weight: bold;
      } /* Keyword */
      code span.op {
        color: #666666;
      } /* Operator */
      code span.ot {
        color: #007020;
      } /* Other */
      code span.pp {
        color: #bc7a00;
      } /* Preprocessor */
      code span.sc {
        color: #4070a0;
      } /* SpecialChar */
      code span.ss {
        color: #bb6688;
      } /* SpecialString */
      code span.st {
        color: #4070a0;
      } /* String */
      code span.va {
        color: #19177c;
      } /* Variable */
      code span.vs {
        color: #4070a0;
      } /* VerbatimString */
      code span.wa {
        color: #60a0b0;
        font-weight: bold;
        font-style: italic;
      } /* Warning */
    </style>
  </head>
  <body>
    <nav>{toc}</nav>
    <main>{body}</main>
  </body>
</html>
