
class TestExtension < Middleman::Extension
  option :all, false, 'Build twice all pages.'
  option :frontmatter_str, "build_twice", 'Tag string on frontmatter.'

  helpers do
    def hogehoge
      if @@second_time
        return yield if block_given?
      else
        @@need_render[current_resource.url] = current_resource
      end
      return ""
    end
  end

  def after_configuration
    app.after_render do |content, path, locs, template_class|
      #sitemap.find_resource_by_path(path).try(:url)
      # restore character entities such as &amp;#96;
      # content ||= ''
      # content.gsub! '&amp;', '&'
      # puts "after render ", locs
      puts "after render #{path}"
      content
    end
    app.after_build do |build|
      # この辺ではもう options は消えているので保存しなきゃいけない
      @@second_time = true
      if @@options[:all]
        for resource in sitemap.resources
          Middleman::Cli::BuildAction.new(build).send(:build_resource, resource)
        end
      else
        @@need_render.each do |url, resource|
          next unless resource
          Middleman::Cli::BuildAction.new(build).send(:build_resource, resource)
        end
      end
    end
  end

  def manipulate_resource_list(resources)
    for resource in resources
      @@need_render[resource.url] = resource if resource.data[options.frontmatter_str]
    end
    resources
  end

  def initialize(app, options_hash={}, &block)
    super
  end
  alias :included :registered
end

::Middleman::Extensions.register(:my_feature, MyFeature)
