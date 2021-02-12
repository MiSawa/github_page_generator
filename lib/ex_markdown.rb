require 'kramdown/parser/kramdown'
class ::Kramdown::Parser::ExMarkdown < ::Kramdown::Parser::GFM

  def initialize(source, options)
    super
    @block_parsers.unshift(:indentation)
    @block_parsers.unshift(:details)
  end
  SPS = "[ \t]"
  CONTAINER_MARK = "^#{OPT_SPACE}:::#{SPS}*"
  ANY_CONTAINER = /#{CONTAINER_MARK}.*?\n/
  INDENTATION_START = /#{CONTAINER_MARK}indent#{SPS}*\n/
  DETAILS_START_EXCEPT_SUMMARY = /#{CONTAINER_MARK}details/
  DETAILS_START = /#{CONTAINER_MARK}details(|#{SPS}+(.*?))#{SPS}*\n/
  CONTAINER_END = /#{CONTAINER_MARK}\n/

  def adapt_source(source)
    source = super
    source = escape_maths source
    source
  end

  def escape_maths(fulldoc)
    dollar              = Regexp.escape "$"
    backslash_dollar    = Regexp.escape "\\$"
    none_escaped        = "(?:(?:^|[^\\\\])(?:\\\\\\\\)*)"
    not_dollar          = "(?:^|$|(?:" + none_escaped + backslash_dollar + ")|(?:[^$]))"
    not_quote           = "(?:^|$|(?:" + none_escaped + Regexp.escape("\\`") + ")|(?:[^`]))"
    not_quote_nor_dollar= "(?:^|$|(?:" + none_escaped + Regexp.escape("\\") + "(?:" + Regexp.escape("`") + "|" + Regexp.escape("$") + ")" + ")|(?:[^`$]))"
    not_quote_none_escaped = "(?:^|$|[^" + Regexp.escape("`\\") + "]" + none_escaped + "?)"
    not_quote_nor_dollar_none_escaped = "(?:^|$|[^$" + Regexp.escape("`\\") + "](?:(?:^|[^\\\\`$])(?:\\\\\\\\)*)?)"

    tex = Regexp.new("(" + not_quote_nor_dollar_none_escaped + ")(" + dollar +
        "(?:|" +
        "(?:" + not_quote_nor_dollar + ")|" + # 1文字
        "(?:" + not_quote_nor_dollar + not_quote_nor_dollar + ")|" + # 2文字
        "(?:" + not_quote_nor_dollar + not_dollar + "*" + not_quote_nor_dollar + ")" +
        ")" + not_quote_nor_dollar_none_escaped + dollar + ")(" + not_quote_nor_dollar + ")")
    tex2 = Regexp.new("(" + not_quote_nor_dollar_none_escaped + ")(" + dollar + dollar +
        "(?:|" +
        not_quote_nor_dollar + "|" + # 1文字
        not_quote_nor_dollar + not_quote_nor_dollar + "|" + # 2文字
        not_quote_nor_dollar + not_dollar + "*" + not_quote_nor_dollar +
        ")" + not_quote_nor_dollar + dollar + dollar + ")(" + not_quote_nor_dollar + ")")
    [tex, tex2].each do |reg|
      loop do
        break unless fulldoc.sub!(reg) do |x|
          $1 + '`' + $2 + '`' + $3
        end
      end
    end
    fulldoc
  end

  def parse_indentation
    start_line_number = @src.current_line_number
    result = []
    nest = 0
    until @src.eos?
      if @src.match? ANY_CONTAINER
        if @src.match? CONTAINER_END
          nest -= 1
        else
          nest += 1
        end
      end
      result << @src.scan(PARAGRAPH_MATCH)
      break if nest.zero?
    end
    unless nest.zero?
      warning("No close tag for indentation (line #{start_line_number}) - auto closing it")
    end

    el = new_block_el(:indentation, nil, nil, location: start_line_number)
    @tree.children << el
    parse_blocks(el, result[1..-2] * '')

    true
  end

  def parse_details
    start_line_number = @src.current_line_number
    @src.match? DETAILS_START
    summary = @src[2]

    el = new_block_el(:details, nil, {summary: summary}, {location: start_line_number})

    if summary
      summary_el = Element.new(:summary, nil, nil, location: start_line_number)
      el.children << summary_el
      env = save_env
      reset_env(:src => Kramdown::Utils::StringScanner.new(summary, start_line_number))
      parse_spans(summary_el)
      p summary
      p summary_el
      restore_env env
    end

    result = []
    nest = 0
    until @src.eos?
      if @src.match? ANY_CONTAINER
        if @src.match? CONTAINER_END
          nest -= 1
        else
          nest += 1
        end
      end
      result << @src.scan(PARAGRAPH_MATCH)
      break if nest.zero?
    end
    unless nest.zero?
      warning("No close tag for details (line #{start_line_number}) - auto closing it")
    end

    parse_blocks(el, result[1..-2] * '')
    @tree.children << el
    true
  end

  define_parser(:indentation, INDENTATION_START)
  define_parser(:details, DETAILS_START)
end

class ::Middleman::Renderers::MiddlemanKramdownHTML < ::Kramdown::Converter::Html
  module KarmdownHtmlExt
    def convert_codeblock(el, indent)
      if el.value[0] == '$' && el.value[-1] == '$'
        el.value
      else
        super el, indent
      end
    end
    def convert_codespan(el, indent)
      if el.value[0] == '$' && el.value[-1] == '$'
        el.value
      else
        super el, indent
      end
    end
    def convert_indentation(el, indent)
      format_as_indented_block_html('div', {'class': 'indent'}, inner(el, indent), indent)
    end

    def convert_summary(el, indent)
      format_as_indented_block_html('summary', {}, inner(el, indent), indent)
    end

    def convert_details(el, indent)
      format_as_indented_block_html('details', {}, inner(el, indent), indent)
    end

    def convert_smart_quote(el, indent)
      case el.value
      when :lsquo, :rsquo then
        return '&#39;'
      when :ldquo, :rdquo then
        return '&quot;'
      end
      super el, indent
    end
  end
  prepend KarmdownHtmlExt
end

