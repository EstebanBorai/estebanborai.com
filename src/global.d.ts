/// <reference types="@sveltejs/kit" />

namespace Domain {
  type BlogNote = {
    title: string;
    description: string;
    date: string;
    categories: string[];
    slug: string;
    preview_image_url: string;
    icon: string;
  };
}
