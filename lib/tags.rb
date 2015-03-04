require 'singleton'

class TagExtension < Middleman::Extension
  option :taglink, '/tags/{tag}.html', 'Path tag pages are generated at.'
  option :tagpage_template, nil, 'Template path (no template extension) for tag archive pages.'

  module TaggedPage
    def tags
      ts = data['tags']
      if ts.is_a? String
        ts.split(',').map(&:strip)
      else
        Array(ts).map(&:to_s)
      end
    end
  end

  class Tag
    attr_reader :tagname, :url
    attr_reader :page, :pages
    def initialize(tagname)
      @manager = Manager.instance
      @tagname = tagname
      @pages = []
      #@url = '/tags/' + tagname
      @url = Middleman::Blog::UriTemplates::apply_uri_template @manager.taglink_template, tag: Middleman::Blog::UriTemplates::safe_parameterize(tagname)
      @url = '/' + @url
    end
    def <=>(other)
      @tagname <=> other.tagname
    end
    def <<(page)
      @pages << page
      self
    end
    def create_resource(app)
      @page = Middleman::Sitemap::Resource.new(app.sitemap,
                                               Middleman::Util::normalize_path(@url)).tap do |p|
        p.proxy_to(@manager.tagpage_template)
        p.add_metadata locals: {
          'page_type' => 'tag',
          'tagname' => @tagname,
          'pages' => @pages,
        }
      end
    end
    def size
      @pages.size
    end
  end

  class Manager
    include Singleton
    attr_reader :tags
    def initialize
      reset
    end

    def reset
      @tags = {}
    end
    attr_reader :tagpage_template, :taglink_template
    def set_options(options)
      @tagpage_template = options.tagpage_template
      @taglink_template = Middleman::Blog::UriTemplates::uri_template options.taglink
    end

    def tagging_pages(resources)
      resources.each{|resource| tagging_page resource }
    end

    def tagging_page(resource)
      return if resource.is_a? TaggedPage
      resource.extend TaggedPage
      resource.tags.each do |tagname|
        @tags[tagname] ||= Tag.new tagname
        @tags[tagname] <<= resource
      end
    end

    def tagpage_resources(app)
      @tags.map{|tagname, tag| tag.create_resource app }
    end
  end

  helpers do
    def tags
      Manager.instance.tags
    end
    def tagpage_resource(tag)
      Manager.instance.tags[tag.strip].page
    end
    def tagpage_url(tag)
      Manager.instance.tags[tag.strip].url
    end
  end

  def initialize(app, option_hash={}, &block)
    super
    @manager = Manager.instance
    @manager.reset
  end

  def after_configuration
    @app.ignore options.tagpage_template if options.tagpage_template
    @manager.set_options(options)
  end

  def manipulate_resource_list(resources)
    @manager.tagging_pages resources
    resources + @manager.tagpage_resources(@app)
  end
end

::Middleman::Extensions.register(:tags, TagExtension)

