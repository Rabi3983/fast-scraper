# Fast Scraper

> Fast Scraper is a high-performance static HTML scraper built for exceptional speed and minimal resource usage.
> Designed for developers who need a lightweight, ultra-efficient scraper, it delivers rapid data extraction with extremely low memory consumption.


<p align="center">
  <a href="https://bitbash.dev" target="_blank">
    <img src="https://github.com/za2122/footer-section/blob/main/media/scraper.png" alt="Bitbash Banner" width="100%"></a>
</p>
<p align="center">
  <a href="https://t.me/devpilot1" target="_blank">
    <img src="https://img.shields.io/badge/Chat%20on-Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white" alt="Telegram">
  </a>&nbsp;
  <a href="https://wa.me/923249868488?text=Hi%20BitBash%2C%20I'm%20interested%20in%20automation." target="_blank">
    <img src="https://img.shields.io/badge/Chat-WhatsApp-25D366?style=for-the-badge&logo=whatsapp&logoColor=white" alt="WhatsApp">
  </a>&nbsp;
  <a href="mailto:sale@bitbash.dev" target="_blank">
    <img src="https://img.shields.io/badge/Email-sale@bitbash.dev-EA4335?style=for-the-badge&logo=gmail&logoColor=white" alt="Gmail">
  </a>&nbsp;
  <a href="https://bitbash.dev" target="_blank">
    <img src="https://img.shields.io/badge/Visit-Website-007BFF?style=for-the-badge&logo=google-chrome&logoColor=white" alt="Website">
  </a>
</p>




<p align="center" style="font-weight:600; margin-top:8px; margin-bottom:8px;">
  Created by Bitbash, built to showcase our approach to Scraping and Automation!<br>
  If you are looking for <strong>Fast Scraper</strong> you've just found your team — Let’s Chat. 👆👆
</p>


## Introduction

Fast Scraper retrieves static website content at scale with impressive speed and efficiency.
It solves the bottlenecks common in traditional scraping tools that rely heavily on slower runtimes.
Ideal for developers, data engineers, and analysts who require high-throughput extraction with minimal overhead.

### Why This Scraper Stands Out

- Built for blazingly fast HTML extraction using a highly optimized backend.
- Requires extremely low memory even at high concurrency levels.
- Handles multiple extraction rules per request with precision.
- Supports flexible global and per-request configuration.
- Optimized for large-scale static website crawling.

## Features

| Feature | Description |
|--------|-------------|
| High-Speed HTML Processing | Extracts static HTML content significantly faster than typical JS-based tools. |
| Low Memory Footprint | Operates efficiently with as little as ~33 MB RAM under load. |
| Configurable Extract Rules | Define CSS selectors, text/HTML extraction types, and attributes. |
| Request-Level Overrides | Customize headers, user agents, and extract rules per request. |
| Robust Retry Logic | Handles failures gracefully with retry timing controls. |
| Optimized Throughput | Supports high concurrency while maintaining stability. |

---

## What Data This Scraper Extracts

| Field Name | Field Description |
|------------|-------------------|
| id | Optional request identifier. |
| url | Target page URL processed by the scraper. |
| field_name | User-defined name for extracted values. |
| selector | CSS selector pointing to the HTML node. |
| extract_type | Extraction type: Text, HTML, or attribute. |
| data | Output object containing extracted results. |

---

## Example Output


    [
        {
            "id": "hockey",
            "url": "https://www.scrapethissite.com/pages/forms/",
            "data": {
                "year2": "\n 1990\n ",
                "class_name": "year",
                "year1": "\n 1990\n "
            }
        },
        {
            "id": "9a2c62e1-79b0-4081-8db8-7d8cf549d4af",
            "url": "https://www.scrapethissite.com/pages/simple/",
            "data": {
                "full_html": "<!doctype html>\n<html lang=\"en\">the rest of html</html>"
            }
        },
        {
            "id": "forms",
            "url": "https://www.scrapethissite.com/pages/simple/",
            "data": {
                "extracted_html": "\n <h3 class=\"country-name\">\n <i class=\"flag-icon flag-icon-ad\"></i>\n Andorra\n </h3>\n <div class=\"country-info\">\n <strong>Capital:</strong> <span class=\"country-capital\">Andorra la Vella</span><br>\n <strong>Population:</strong> <span class=\"country-population\">84000</span><br>\n <strong>Area (km<sup>2</sup>):</strong> <span class=\"country-area\">468.0</span><br>\n </div>\n "
            }
        }
    ]

---

## Directory Structure Tree


    Fast Scraper/
    ├── src/
    │   ├── main.rs
    │   ├── scraper/
    │   │   ├── html_parser.rs
    │   │   ├── selector_engine.rs
    │   │   └── request_handler.rs
    │   ├── config/
    │   │   └── settings.example.json
    │   └── utils/
    │       └── retry_logic.rs
    ├── data/
    │   ├── sample_output.json
    │   └── input_examples.json
    ├── Cargo.toml
    └── README.md

---

## Use Cases

- **Data analysts** scrape thousands of static pages to rapidly collect structured datasets for research and modelling.
- **SEO specialists** extract metadata and on-page content at scale to audit website performance.
- **Developers** integrate the scraper into backend pipelines to automate content collection for search indices.
- **Market researchers** gather product or pricing info efficiently without heavy resource costs.
- **Engineers** use it to benchmark static site performance across large URL batches.

---

## FAQs

**Q: Does this scraper support JavaScript-rendered pages?**
A: No, Fast Scraper is optimized for static HTML only. For JS-rendered sites, use tools designed for dynamic rendering.

**Q: Can I define custom headers for each request?**
A: Yes, request-specific headers override global headers, giving you full control over your scraping strategy.

**Q: What happens if a request fails?**
A: You can configure retry count, retry timeout, and delay between attempts to ensure reliability.

**Q: Can I extract attributes instead of text or HTML?**
A: Yes, attributes such as `class`, `href`, or `src` can be extracted through the `extract_type` configuration.

---

## Performance Benchmarks and Results

**Primary Metric:** Processes ~1,000 static pages in ~60 seconds under moderate concurrency.
**Reliability Metric:** Maintains stable extraction with controlled retry mechanisms and low failure rate.
**Efficiency Metric:** Operates at an average of 33.2 MB RAM and <1% CPU in benchmark conditions.
**Quality Metric:** Provides complete HTML extraction with high selector accuracy and consistent field output.


<p align="center">
<a href="https://calendar.app.google/74kEaAQ5LWbM8CQNA" target="_blank">
  <img src="https://img.shields.io/badge/Book%20a%20Call%20with%20Us-34A853?style=for-the-badge&logo=googlecalendar&logoColor=white" alt="Book a Call">
</a>
  <a href="https://www.youtube.com/@bitbash-demos/videos" target="_blank">
    <img src="https://img.shields.io/badge/🎥%20Watch%20demos%20-FF0000?style=for-the-badge&logo=youtube&logoColor=white" alt="Watch on YouTube">
  </a>
</p>
<table>
  <tr>
    <td align="center" width="33%" style="padding:10px;">
      <a href="https://youtu.be/MLkvGB8ZZIk" target="_blank">
        <img src="https://github.com/za2122/footer-section/blob/main/media/review1.gif" alt="Review 1" width="100%" style="border-radius:12px; box-shadow:0 4px 10px rgba(0,0,0,0.1);">
      </a>
      <p style="font-size:14px; line-height:1.5; color:#444; margin:0 15px;">
        “Bitbash is a top-tier automation partner, innovative, reliable, and dedicated to delivering real results every time.”
      </p>
      <p style="margin:10px 0 0; font-weight:600;">Nathan Pennington
        <br><span style="color:#888;">Marketer</span>
        <br><span style="color:#f5a623;">★★★★★</span>
      </p>
    </td>
    <td align="center" width="33%" style="padding:10px;">
      <a href="https://youtu.be/8-tw8Omw9qk" target="_blank">
        <img src="https://github.com/za2122/footer-section/blob/main/media/review2.gif" alt="Review 2" width="100%" style="border-radius:12px; box-shadow:0 4px 10px rgba(0,0,0,0.1);">
      </a>
      <p style="font-size:14px; line-height:1.5; color:#444; margin:0 15px;">
        “Bitbash delivers outstanding quality, speed, and professionalism, truly a team you can rely on.”
      </p>
      <p style="margin:10px 0 0; font-weight:600;">Eliza
        <br><span style="color:#888;">SEO Affiliate Expert</span>
        <br><span style="color:#f5a623;">★★★★★</span>
      </p>
    </td>
    <td align="center" width="33%" style="padding:10px;">
      <a href="https://youtube.com/shorts/6AwB5omXrIM" target="_blank">
        <img src="https://github.com/za2122/footer-section/blob/main/media/review3.gif" alt="Review 3" width="35%" style="border-radius:12px; box-shadow:0 4px 10px rgba(0,0,0,0.1);">
      </a>
      <p style="font-size:14px; line-height:1.5; color:#444; margin:0 15px;">
        “Exceptional results, clear communication, and flawless delivery. Bitbash nailed it.”
      </p>
      <p style="margin:10px 0 0; font-weight:600;">Syed
        <br><span style="color:#888;">Digital Strategist</span>
        <br><span style="color:#f5a623;">★★★★★</span>
      </p>
    </td>
  </tr>
</table>
