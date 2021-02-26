site_url = "https://misawa.github.io"
xml.instruct!
xml.urlset 'xmlns' => "http://www.sitemaps.org/schemas/sitemap/0.9" do
  sitemap.resources.select{|page| page.destination_path =~ /\.html$/}.each do |page|
    xml.url do
      xml.loc URI.escape(File.join(site_url, page.destination_path))
      xml.lastmod page.mtime.iso8601
      xml.changefreq "weekly"
      xml.priority "0.5"
    end
  end
end
