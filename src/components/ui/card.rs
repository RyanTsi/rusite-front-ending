use leptos::prelude::*;
use leptos_icons::Icon;
use icondata as i;
use crate::{components::ui::{button::{Button, Link}, icon::DividingLine}, models::blog::{Article, ArticleInfo, Category, Tag}, state::{use_app, FilterBarState}, utils::{format_date_cn, render_markdown_with_toc}};

#[component]
fn Card(
    /// 卡片变体样式（color）
    #[prop(optional, default = CardVariant::Default)]
    variant: CardVariant,
    /// 阴影级别 (0-5)
    #[prop(optional, default = CardElevation::Md)]
    elevation: CardElevation,
    /// 聚焦时阴影级别 (0-5)
    #[prop(optional, default = CardElevation::Lg)]
    h_elevation: CardElevation,
    /// 内边距大小
    #[prop(optional, default = CardPadding::Md)]
    padding: CardPadding,
    /// 圆角大小
    #[prop(optional, default = CardRadius::Md)]
    radius: CardRadius,
    /// 
    #[prop(optional, into)]
    class: String,
    /// 主体内容
    children: Children,
) -> impl IntoView {
    // 获取变体类名
    let variant_class = variant.as_class();
    // 阴影类名
    let elevation_class = elevation.as_class();
    let h_elevation_class = h_elevation.as_class();
    // 圆角类名
    let radius_class = radius.as_class();
    // 内边距类名
    let padding_class = padding.as_class();

    let color = "bg-white";
    // 组合所有类名
    let card_classes = format!(
        "{} {} {} {} hover:{} transition-all duration-300 {}",
        padding_class,
        color,
        radius_class,
        elevation_class,
        h_elevation_class,
        class,
    );
    // let card_classes = "p-8 bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow duration-200 w-2/3";

    view! {
        <div class=card_classes>
            {children()}
        </div>
    }
}

/// 卡片变体样式
#[derive(Clone, Default)]
pub enum CardVariant {
    /// 默认样式
    #[default]
    Default,
    /// 强调样式
    Primary,
    /// 轮廓样式
    Outline,
    /// 无边框样式
    Ghost,
}


impl CardVariant {
    // TODO: 
    fn as_class(&self) -> &'static str {
        match self {
            CardVariant::Default => "",
            CardVariant::Primary => "bg-primary-50 dark:bg-primary-900/20",
            CardVariant::Outline => "border border-gray-300 dark:border-gray-600",
            CardVariant::Ghost => "bg-transparent shadow-none",
        }
    }
}

/// 卡片内边距选项
#[derive(Clone, Default)]
pub enum CardPadding {
    None,
    #[default]
    Sm,
    Md,
    Lg,
    Xl,
}

impl CardPadding {
    fn as_class(&self) -> &'static str {
        match self {
            CardPadding::Sm => "p-4",
            CardPadding::Md => "p-6",
            CardPadding::Lg => "p-8",
            CardPadding::Xl => "p-12",
            CardPadding::None => "",
        }
    }
}

/// 卡片阴影选项
#[derive(Clone, Default)]
pub enum CardElevation {
    None,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Xxl,
    Xxxl,
}
impl CardElevation {
    fn as_class(&self) -> &'static str {
        match self {
            CardElevation::Sm => "shadow-sm",
            CardElevation::Md => "shadow-md",
            CardElevation::Lg => "shadow-lg",
            CardElevation::Xl => "shadow-xl",
            CardElevation::Xxl => "shadow-2xl",
            CardElevation::Xxxl => "shadow-3xl",
            CardElevation::None => "",
        }
    }
}

/// 卡片圆角选项
#[derive(Clone, Default)]
pub enum CardRadius {
    None,
    Sm,
    #[default]
    Md,
    Lg,
    Full,
}
impl CardRadius {
    fn as_class(&self) -> &'static str {
        match self {
            CardRadius::Sm => "rounded-sm",
            CardRadius::Md => "rounded-md",
            CardRadius::Lg => "rounded-lg",
            CardRadius::Full => "rounded-full",
            CardRadius::None => "",
        }
    }
}

#[component]
pub fn ArticleInfoCard(
    info: ArticleInfo,
) -> impl IntoView {
    let url = format!("/blog/{}", info.aid);
    view! {
        <Card>
            <div class="flex flex-row justify-between">
                <h1 class = "text-3xl font-bold">
                    {info.title}
                </h1>
                <p class = "text-gray-400 text-sm">
                    {format_date_cn(info.created_at)}
                </p>
            </div>
            <div class="my-4">
                <p class="text-gray-600">
                    {info.summary}
                </p>
            </div>
            <div class="flex flex-row gap=4 justify-between items-center">
                <div class="flex flex-row gap-2 items-center">
                    <Icon icon={i::FaFolderOpenSolid}/>
                    <div id="categories" class="flex flex-wrap gap-2">
                        <For
                            each=move || info.categories.clone()
                            key=|tag| tag.clone()
                            children=move |tag| {
                                view! {
                                    <span class="px-3 py-1 bg-blue-100 text-blue-700 rounded-full text-sm">
                                        {tag}
                                    </span>
                                }
                            }
                        />
                    </div>
                    <Icon icon={i::FaTagsSolid}/>
                    <div id="tags" class="flex flex-wrap gap-2">
                        <For
                            each=move || info.tags.clone()
                            key=|tag| tag.clone()
                            children=move |tag| {
                                view! {
                                    <span class="px-3 py-1 bg-blue-100 text-blue-700 rounded-full text-sm">
                                        {tag}
                                    </span>
                                }
                            }
                        />
                    </div>
                </div>
                <div>
                    <Link href=url>
                        <div class="flex flex-row items-center justify-center gap-1">
                            <p> "Read" </p>
                            <Icon icon={i::FaChevronRightSolid}/>
                        </div>
                    </Link>
                </div>
            </div>
        </Card>
    }
}

#[component]
pub fn ArticleCard(
    article: Article
) -> impl IntoView {
    let (a, _) = render_markdown_with_toc(article.content());
    view! {
        <Card class="flex mx-auto w-2/3 h-screen">
            <div class="flex flex-col gap-4 w-full">
                <div id="header" class="flex flex-col gap-4">
                    <div class="flex flex-row items-center text-2xl gap-8">
                        <Link href="/blog".to_string()>
                            <Icon icon={i::FaChevronLeftSolid}/>
                        </Link>
                        <h1> {article.title().to_string()} </h1>
                    </div>
                    <div class="flex flex-row items-center gap-8">
                        <div class="flex flex-row items-center text-sm text-gray-600 gap-2">
                            <Icon icon={i::FaCalendarDaysSolid}/>
                            <p> {article.created_at().to_string()} </p>
                        </div>
                        <div class="flex flex-row items-center text-sm text-gray-600 gap-2">
                            <Icon icon={i::FaCompassDraftingSolid}/>
                            <p> {article.updated_at().to_string()} </p>
                        </div>
                        <div class="flex flex-row items-center text-sm text-gray-600 gap-2">
                            <Icon icon={i::FaFolderOpenSolid}/>
                            <p> {article.tags().join(" | ")} </p>
                        </div>
                        <div class="flex flex-row items-center text-sm text-gray-600 gap-2">
                            <Icon icon={i::FaTagsSolid}/>
                            <p> {article.categories().join(" | ")} </p>
                        </div>
                    </div>
                </div>
                <DividingLine/>

                <div inner_html={a}></div>
            </div>
        </Card>
    }
}

#[component]
pub fn FilterBarCard(
    state: RwSignal<FilterBarState>
) -> impl IntoView {
    view! {
        <Card>
            <p>Select:</p>
            // 显示已选择的tags
            <Show when=move || !state.get().selected_tags.get().is_empty()>
                <div class="mb-4">
                    <div class="flex flex-wrap gap-2 mt-2">
                        <For 
                            each={ move || state.get().selected_tags.get().into_iter().collect::<Vec<_>>() }
                            key=move |tag: &String| tag.clone()
                            children=move |tag| {
                                let tag_name = tag.clone();
                                view! {
                                    <Button
                                        on_click=Callback::new(move |_| {
                                            state.get().remove_tag(tag.clone());
                                        })
                                    >
                                        {tag_name}
                                    </Button>
                                }
                            }
                        />
                    </div>
                </div>
            </Show>
            <Show when=move || !state.get().selected_categories.get().is_empty()>
                <div class="mb-4">
                    <div class="flex flex-wrap gap-2 mt-2">
                        <For 
                            each={ move || state.get().selected_categories.get().into_iter().collect::<Vec<_>>() }
                            key=move |category: &String| category.clone()
                            children=move |category| {
                                let category_name = category.clone();
                                view! {
                                    <Button
                                        on_click=Callback::new(move |_| {
                                            state.get().remove_category(category.clone());
                                        })
                                    >
                                        {category_name}
                                    </Button>
                                }
                            }
                        />
                    </div>
                </div>
            </Show>
            <DividingLine/>
            
            <div class="flex flex-wrap gap-4">
                <For
                    each=move || state.get().tags.get()
                    key=move |tag| tag.name.clone()
                    children=move |tag| {
                        let tag_name = tag.name.clone();
                        view! {
                            <Button
                                on_click=Callback::new(move |_| {
                                    state.get().switch_tag_selected(tag.name.clone());
                                })
                            >
                                {tag_name}
                            </Button>
                        }
                    }
                />
            </div>
            <DividingLine/>
            <div class="flex flex-wrap gap-4">
                <For
                    each=move || state.get().categories.get()
                    key=move |category| category.name.clone()
                    children=move |category| {
                        let category_name = category.name.clone();
                        view! {
                                <Button
                                    on_click=Callback::new(move |_| {
                                        state.get().switch_category_selected(category.name.clone());
                                    })
                                >
                                    {category_name}
                                </Button>
                            }
                        }
                />
            </div>
        </Card>
    }
}