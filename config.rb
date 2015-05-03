###
# Compass
###

# Change Compass configuration
# compass_config do |config|
#   config.output_style = :compact
# end

###
# Page options, layouts, aliases and proxies
###

# Per-page layout changes:
#
# With no layout
# page "/path/to/file.html", :layout => false
#
# With alternative layout
# page "/path/to/file.html", :layout => :otherlayout
#
# A path which all have the same layout
# with_layout :admin do
#   page "/admin/*"
# end

# Proxy pages (http://middlemanapp.com/basics/dynamic-pages/)
# proxy "/this-page-has-no-template.html", "/template-file.html", :locals => {
#  :which_fake_page => "Rendering a fake page with a local variable" }

###
# Helpers
###

# Automatic image dimensions on image_tag helper
# activate :automatic_image_sizes

# Reload the browser automatically whenever files change
# configure :development do
  activate :livereload
# end

# Methods defined in the helpers block are available in templates
# helpers do
#   def some_helper
#     "Helping"
#   end
# end

set :css_dir, 'stylesheets'

set :js_dir, 'javascripts'

set :images_dir, 'images'

# Build-specific configuration
configure :build do
  # For example, change the Compass output style for deployment
  activate :minify_css # sass, 勝手にコメントで filepath 埋め込むの死んでくれ.

  # Minify Javascript on build
  # activate :minify_javascript

  # Enable cache buster
  # activate :asset_hash

  # Use relative URLs
  # activate :relative_assets

  # Or use a different image path
  # set :http_prefix, "/Content/images/"
end


#set :markdown_engine, :kramdown
#set :markdown, :parse_block_html => true


# markdown engine
# https://gist.github.com/plusjade/2699636
require 'redcarpet'
require 'middleman-core/renderers/redcarpet'
class HTMLWithMathjax < Middleman::Renderers::MiddlemanRedcarpetHTML
  #require 'rouge'
  #require 'rouge/plugins/redcarpet'
  #include Rouge::Plugins::Redcarpet
  #def block_code(code, language)
  #  #Middleman::Syntax::Highlighter.highlight(code, language)
  #  if language == 'mathjax'
  #    "<script type=\"math/tex; mode=display\">
  #    #{code}
  #    </script>"
  #  else
  #    "<pre><code class=\"#{language}\">#{code}</code></pre>"
  #  end
  #end
  def codespan(code)
    if code[0] == "$" && code[-1] == "$"
      code
      #code.gsub!(/^\$/,'')
      #code.gsub!(/\$$/,'')
      #"<script type=\"math/tex\">#{code}</script>"
    else
      "<code>#{code}</code>"
    end
  end
end
#set :markdown_engine, :redcarpet
#set :markdown, :fenced_code_blocks => true, :smartypants => true,
#  :renderer => HTMLWithMathjax
set :markdown_engine, :redcarpet
set :markdown, :fenced_code_blocks => true, :smartypants => true, :renderer => HTMLWithMathjax

activate :syntax, :line_numbers => true

activate :relative_assets
set :relative_links, true
set :strip_index_file, false

# TODO config.rb から, ページ指定して書けるようにしたいね.

require 'lib/build_twice'
activate :build_twice

require 'lib/fix_url'
require 'lib/tags'
require 'lib/secret'

activate :secret # tags より前に activate すること.
activate :tags, :tagpage_template => 'tags/tag_template.html'


class SocialButton
  attr_reader :js, :html, :style
  def initialize(js, html, style)
    @js, @html, @style = js, html, style
  end
@@tweet_button = SocialButton.new(<<JS, <<HTML, <<STYLE)
<script>!function(d,s,id){var js,fjs=d.getElementsByTagName(s)[0],p=/^http:/.test(d.location)?'http':'https';if(!d.getElementById(id)){js=d.createElement(s);js.id=id;js.src=p+'://platform.twitter.com/widgets.js';fjs.parentNode.insertBefore(js,fjs);}}(document, 'script', 'twitter-wjs');</script>
JS
<a href="https://twitter.com/share" class="twitter-share-button" data-via="Mi_Sawa" data-dnt="true">Tweet</a>
HTML
STYLE
@@hatena_star = SocialButton.new(<<JS, <<HTML, <<STYLE)
<script type="text/javascript" src="http://s.hatena.ne.jp/js/HatenaStar.js"></script>
<script type="text/javascript">
Hatena.Star.Token = 'bd3a38c3b5f80b3ffb7ac27a6d10b8a144d8a948';
Hatena.Star.SiteConfig = {
  entryNodes: {
    'div#main' : {
      uri: 'document.location',
      title: 'document.title',
      container: 'span.hatena_star'
    }
  }
};
</script>
JS
<span class="hatena_star"></span>
HTML
STYLE
  def SocialButton.tweet_button
    @@tweet_button
  end
  def SocialButton.hatena_star
    @@hatena_star
  end
end

helpers do
  def social_buttons
    return [ SocialButton::hatena_star, SocialButton::tweet_button ]
  end
end

helpers do
  def get_title(resource)
    title = resource.metadata[:title]
    title ||= resource.data.title
    title ||= resource.url
    title += " - みさわめも"
  end
end


page '/aoj/*', :layout => 'aoj'
page '/aoj/index.html', :layout => 'layout'

page '/topcoder/*', :layout => 'topcoder'
page '/topcoder/index.html', :layout => 'layout'

page '/yukicoder/*', :layout => 'yukicoder'
page '/yukicoder/index.html', :layout => 'layout'

#ignore '/secret/*'

