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
  activate :minify_css

  # Minify Javascript on build
  # activate :minify_javascript

  # Enable cache buster
  activate :asset_hash

  # Use relative URLs
  # activate :relative_assets

  # Or use a different image path
  # set :http_prefix, "/Content/images/"
end


activate :sprockets do |c|
  c.expose_middleman_helpers = true
end

require 'lib/ex_markdown'
set :markdown_engine, :kramdown
set :markdown,
  smartypants: false,
  tables: true,
  auto_ids: false,
  typographic_symbols: {
    hellip: '...',
  },
  input: :ExMarkdown,
  hard_wrap: false


activate :syntax, :line_numbers => true

activate :relative_assets
set :relative_links, true
set :strip_index_file, false

require 'lib/fix_url'
require 'lib/tags'
require 'lib/secret'

activate :tags, :tagpage_template => 'tags/tag_template.html'
configure :build do
  activate :secret
end


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
<script type="text/javascript" src="https://s.hatena.ne.jp/js/HatenaStar.js"></script>
<script type="text/javascript">
Hatena.Star.Token = 'bd3a38c3b5f80b3ffb7ac27a6d10b8a144d8a948';
Hatena.Star.SiteConfig = {
  entryNodes: {
    'main' : {
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
    #return [ SocialButton::hatena_star, SocialButton::tweet_button ]
    return []
  end
end

helpers do
  def get_title(resource)
    title = resource.metadata[:title]
    layout = resource.metadata[:options][:layout]
    problem = resource.data.problem
    case layout
    when 'aoj' then
      return "AOJ #{problem.id} #{problem.name}"
    when 'topcoder' then
      if problem
        return "TopCoder #{problem.round} #{problem.level * ','} #{problem.name}"
      else
        return "TopCoder #{resource.data.contest.round}"
      end
    when 'yukicoder' then
      id = problem.id
      id = id.to_i if id and id.class == String
      return "yukicoder #{'%04d' % id} #{problem.name}"
    when 'atcoder' then
      return "#{problem.contest} #{problem.id} #{problem.name}"
    else
      title ||= resource.data.title
      title ||= resource.url
    end
    title
  end
end

activate :deploy do |deploy|
  # deploy.deploy_method = :git
  # deploy.branch = 'master'
  set :build_dir, 'MiSawa.github.io'
end


page '/aoj/index.html', :layout => 'layout'
page '/aoj/*', :layout => 'aoj'

page '/topcoder/index.html', :layout => 'layout'
page '/topcoder/*', :layout => 'topcoder'

page '/yukicoder/index.html', :layout => 'layout'
page '/yukicoder/*', :layout => 'yukicoder'

page '/atcoder/index.html', :layout => 'layout'
page '/atcoder/*', :layout => 'atcoder'

configure :build do
  ignore '/secret/*'
end

# README を置いておくとそれを README.md にリネームして build ディレクトリに置く.
class IgnoreReadme < Middleman::Extension
  def initialize(app, options_hash={}, &block)
    super
  end
  def manipulate_resource_list(resources)
    resources.each do |resource|
      if resource.destination_path == 'README'
        resource.destination_path = 'README.md'
      end
    end
  end
end
::Middleman::Extensions.register(:ignore_readme, IgnoreReadme)
activate :ignore_readme


