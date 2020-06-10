class BuildTwiceExtension < Middleman::Extension
  @@second_time = false
  option :all, false, 'Build twice all resources.'
  option :frontmatter_str, "build_twice", 'Tag string on frontmatter.'

  helpers do
    def build_twice
      if @@second_time
        return yield if block_given?
      else
        @@need_build[current_resource.url] = current_resource
      end
      return ""
    end
  end

  def manipulate_resource_list(resources)
    for resource in resources
      if resource.data[options.frontmatter_str]
        @@need_build[resource.url] = resource
      end
    end
    resources
  end

  def after_configuration
    return if @@second_time
    @@need_build = {}
    @@options = {}
    @@options[:all] = options[:all]
    @@options[:frontmatter_str] = options[:frontmatter_str]
    app.after_build do |build|
      @@second_time = true
      # middleman-core/lib/middleman-core/cli/build.rb
      if @@options[:all]
        for resource in sitemap.resources
          next if resource.ignored?
          Middleman::Cli::Build.new().send(:build)
        end
      else
        @@need_build.each do |url, resource|
          next unless resource
          next if resource.ignored?
          Middleman::Cli::Build.new().send(:build)
        end
      end
    end
  end

  def initialize(app, options_hash={}, &block)
    super
  end
end

::Middleman::Extensions.register(:build_twice, BuildTwiceExtension)

