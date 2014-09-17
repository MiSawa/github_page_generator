require 'addressable/uri'
module Middleman
  module Util
    class << self
      # Given a source path (referenced either absolutely or relatively)
      # or a Resource, this will produce the nice URL configured for that
      # path, respecting :relative_links, directory indexes, etc.
      def url_for(app, path_or_resource, options={})
        # Handle Resources and other things which define their own url method
        url = if path_or_resource.respond_to?(:url)
          path_or_resource.url
        else
          path_or_resource.dup
        end.gsub(' ', '%20')

        # Try to parse URL
        begin
          uri = Addressable::URI.parse(url)
        rescue Addressable::URI::InvalidURIError
          # Nothing we can do with it, it's not really a URI
          return url
        end

        relative = options[:relative]
        raise "Can't use the relative option with an external URL" if relative && uri.host

        # Allow people to turn on relative paths for all links with
        # set :relative_links, true
        # but still override on a case by case basis with the :relative parameter.
        effective_relative = relative || false
        effective_relative = true if relative.nil? && app.config[:relative_links]

        # Try to find a sitemap resource corresponding to the desired path
        this_resource = options[:current_resource]

        if path_or_resource.is_a?(::Middleman::Sitemap::Resource)
          resource = path_or_resource
          resource_url = url
        elsif this_resource && uri.path
          # Handle relative urls
          url_path = Pathname(uri.path)
          current_source_dir = Pathname('/' + this_resource.path).dirname
          url_path = current_source_dir.join(url_path) if url_path.relative?
          resource = app.sitemap.find_resource_by_path(url_path.to_s)
          resource_url = resource.url if resource
        elsif options[:find_resource] && uri.path
          resource = app.sitemap.find_resource_by_path(uri.path)
          resource_url = resource.url if resource
        end

        if resource
          uri.path = relative_path_from_resource(this_resource, resource_url, effective_relative)
        else
          # If they explicitly asked for relative links but we can't find a resource...
          raise "No resource exists at #{url}" if relative
        end

        # Support a :query option that can be a string or hash
        if query = options[:query]
          uri.query = query.respond_to?(:to_param) ? query.to_param : query.to_s
        end

        # Support a :fragment or :anchor option just like Padrino
        fragment = options[:anchor] || options[:fragment]
        uri.fragment = fragment.to_s if fragment

        # Finally make the URL back into a string
        uri.to_s
      end
    end
  end
end
