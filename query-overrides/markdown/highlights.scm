[
  (atx_h1_marker)
  (atx_h2_marker)
  (atx_h3_marker)
  (atx_h4_marker)
  (atx_h5_marker)
  (atx_h6_marker)
  (setext_h1_underline)
  (setext_h2_underline)
] @keyword

(atx_heading
  (heading_content) @markup.heading)

(setext_heading
  (heading_content) @markup.heading)

(thematic_break) @punctuation.special

(fenced_code_block
  (info_string) @label)

(link_destination) @markup.link.url
