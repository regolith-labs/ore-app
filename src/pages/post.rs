use dioxus::prelude::*;

use crate::{
    components::*,
    gateway::ore::{OreGateway, WaitlistStatus},
    hooks::{use_gateway, use_wallet, Wallet},
    route::Route,
    components::CheckCircleIcon,
};

use chrono::{DateTime, Utc, Local, Duration, FixedOffset};
use serde::{Deserialize, Serialize};


pub fn Post() -> Element {
    let wallet = use_wallet();

    // Check waitlist status if wallet is connected
    let waitlist_status = use_resource(move || async move {
        if let Wallet::Connected(pubkey) = *wallet.read() {
            use_gateway().validate_waitlist_status(pubkey).await
        } else {
            Ok(WaitlistStatus {
                exists: false,
                user_id: "".to_string(),
                is_registered: false,
                screen_name: None,
                waitlist_number: None,
                profile_image_url: None,
                is_approved: false,
            })
        }
    });

    rsx! {
        match *wallet.read() {
            Wallet::Connected(_) => {                
                match waitlist_status.read().as_ref() {
                    Some(Ok(status)) => {
                        log::info!("Waitlist status: {:?}", status);
                        if status.is_registered && status.is_approved {
                            rsx! { Dashboard {
                                user_id: status.user_id.clone(),
                                screen_name: match &status.screen_name {
                                    Some(name) => name.clone(),
                                    None => "@user".to_string()
                                },
                                profile_image_url: match &status.profile_image_url {
                                    Some(url) => url.clone(),
                                    None => "https://via.placeholder.com/150".to_string()
                                }
                            } }
                        } else if status.is_registered && !status.is_approved {
                            rsx! { Waitlist { status: status.clone() } }
                        } else {
                            rsx! { Onboarding {} }
                        }
                    },
                    Some(Err(err)) => {
                        log::error!("Error fetching waitlist status: {:?}", err);
                        rsx! { div { class: "mx-auto w-full max-w-2xl px-5 sm:px-8", "Loading..." } }
                    },
                    None => rsx! { 
                        Col {
                            class: "w-full h-full pb-20 sm:pb-16",
                            gap: 8,
                            Heading {
                                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                                title: "Loading...",
                                subtitle: "Please wait while we fetch your status."
                            }
                            div {
                                class: "mx-auto w-full max-w-2xl px-5 sm:px-8 flex justify-center",
                                div {
                                    class: "animate-spin rounded-full h-8 w-8 border-b-2 border-elements-highEmphasis"
                                }
                            }
                        }
                    }
                }
            },
            _ => rsx! { Onboarding {} }
        }
    }
}

fn LoadingOnboarding() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Post",
                subtitle: "Get paid to create and share content."
            }
            Col {
                gap: 8,
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                span {
                    class: "text-lg",
                    "Eligible creators will soon be able to earn ORE by creating and sharing engaging content on X dot com. To join the waitlist, log in with your X account below."
                }
                Col {
                    gap: 4,
                    SignInWithX {}
                    span {
                        class: "text-xs text-elements-lowEmphasis text-center",
                        "By logging in with X, you agree to share your data with Regolith Labs and accept the "
                        Link {
                            class: "underline",
                            to: Route::PostTerms {},
                            "Terms and Conditions"
                        }
                        " of the ORE Creator Program. Regolith Labs may modify or disable the ORE Creator Program at any time in its sole discretion, including for business, financial, or legal reasons."
                    }
                }
            }
        }
    }
}

fn Onboarding() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Post",
                subtitle: "Get paid to create and share content."
            }
            Col {
                gap: 8,
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                span {
                    class: "text-lg",
                    "Eligible creators will soon be able to earn ORE by creating and sharing engaging content on X dot com. To join the waitlist, log in with your X account below."
                }
                Col {
                    gap: 4,
                    SignInWithX {}
                    span {
                        class: "text-xs text-elements-lowEmphasis text-center",
                        "By logging in with X, you agree to share your data with Regolith Labs and accept the "
                        Link {
                            class: "underline",
                            to: Route::PostTerms {},
                            "Terms and Conditions"
                        }
                        " of the ORE Creator Program. Regolith Labs may modify or disable the ORE Creator Program at any time in its sole discretion, including for business, financial, or legal reasons."
                    }
                }
            }
        }
    }
}

fn SignInWithX() -> Element {
    let wallet = use_wallet();
    let request_token = use_resource(|| async move { use_gateway().get_x_request_token().await });

    let is_wallet_connected = matches!(*wallet.read(), Wallet::Connected(_));

    rsx! {
        if let Some(Ok(token)) = request_token.cloned() {
            div {
                if is_wallet_connected {
                    a {
                        class: "controls-primary w-full flex flex-row justify-center items-center gap-1.5 rounded-full h-12",
                        href: format!("https://api.x.com/oauth/authenticate?oauth_token={}", token),
                        target: "_blank",
                        rel: "noopener noreferrer",
                        span { "Log in with " },
                        XIcon { class: "w-4 h-4" }
                    }
                } else {
                    button {
                        class: "controls-primary w-full flex flex-row justify-center items-center gap-1.5 rounded-full h-12 opacity-70 cursor-not-allowed",
                        disabled: true,
                        span { "Log in with " },
                        XIcon { class: "w-4 h-4" }
                    }
                }
            }
        } else {
            div { "Loading..." }
        }
    }
}

#[component]
fn Waitlist(status: WaitlistStatus) -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Post",
                subtitle: "Get paid to create and share content."
            }
            Col {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                gap: 4,
                {status.profile_image_url.as_ref().map(|url| rsx! {
                    div {
                        class: "flex justify-center mb-3",
                        img {
                            src: "{url}",
                            class: "w-16 h-16 rounded-full",
                            alt: "Profile image"
                        }
                    }
                })}
                {
                    if let (Some(_name), Some(number)) = (&status.screen_name, status.waitlist_number) {
                        rsx! {
                            span {
                                class: "text-elements-highEmphasis font-semibold text-2xl mx-auto",
                                "You're #{number} on the waitlist!"
                            }
                        }
                    } else {
                        rsx! {
                            span {
                                class: "text-elements-highEmphasis font-semibold text-2xl mx-auto",
                                "You're on the waitlist!"
                            }
                        }
                    }
                }
                span {
                    class: "text-elements-midEmphasis font-medium mx-auto text-center",
                    "The ORE Creator Program will be launching soon. Follow @OREsupply on X and check back soon for updates."
                }
            }
        }
    }
}


fn NetDeposits() -> Element {
    rsx! {
        Col {
            gap: 4,
            span {
                class: "text-elements-lowEmphasis font-medium text-right",
                "Current score"
            }
            span {
                class: "text-3xl font-semibold text-elements-highEmphasis text-right",
                "100"
            }
        }
    }
}

fn NetYield() -> Element {
    // let net_yield = use_net_yield();
    rsx! {
        Col {
            // class: "min-w-56",
            gap: 4,
            span {
                class: "text-elements-lowEmphasis font-medium text-right",
                "Total earnings"
            }
            OreValue {
                class: "text-center",
                ui_amount_string: "100",
                with_decimal_units: true,
                size: TokenValueSize::Large,
                gold: true,
            }
            // if let Ok(net_yield) = net_yield.cloned() {
            //     OreValue {
            //         class: "md:text-right md:ml-auto",
            //         ui_amount_string: net_yield.ui_amount_string,
            //         with_decimal_units: true,
            //         size: TokenValueSize::Large,
            //         gold: true,
            //     }
            // } else {
            //     LoadingValue {}
            // }
        }
    }
}

fn ClaimButton() -> Element {
    rsx! {
        button {
            disabled: false,
            onclick: move |_| {
                // TODO: Handle form submission
            },
            class: "flex flex-row h-12 w-full controls-gold rounded-full px-8 justify-center",
            span {
                class: "my-auto mx-auto text-nowrap",
                "Claim"
            }
        }
    }
}


#[component]
fn ProfileInfo(screen_name: String, profile_image_url: String) -> Element {
    rsx! { 
        Col {
            gap: 4,                
            img {
                src: profile_image_url,
                class: "w-16 h-16 rounded-full",
                alt: "Profile image"
            }
            Subheading {
                title: format!("@{}", screen_name)
            }
        }
    }
}

#[component]
fn AccountSummary(user_id: String, screen_name: String, profile_image_url: String) -> Element {
    rsx! {
        Col {
            class: "mx-auto w-full px-5 max-w-2xl sm:px-8 justify-between",
            gap: 8,
            Row {
                class: "w-full justify-between",
                ProfileInfo { screen_name, profile_image_url }                    
                Col {
                    class: "items-end justify-end",
                    gap: 8,
                    Row {
                        gap: 8,
                        NetDeposits {}
                        NetYield {}
                    }                                    
                }            
            }
            Row {
                class: "w-full",
                ClaimButton {}
            }
        }        
    }
}

fn CreatorInfo() -> Element {
    rsx! {
        Col {
            gap: 4,             
            img {
                src: "https://avatars.githubusercontent.com/u/583231?v=4",
                class: "w-16 h-16 rounded-full",
                alt: "Profile image"
            }
            Subheading {
                title: "@iocozmo"
            }           
        }
    }
}

#[component]
fn PostForm(username: Option<String>) -> Element {  
    let mut url = use_signal::<String>(|| "".to_string());
    let mut url_err = use_signal::<Option<String>>(|| None);

    rsx! {
        Col {
            class: "w-full",
            gap: 4,
            Col {
                class: "w-full lg:flex elevated elevated-border shrink-0 h-min rounded-xl z-0",
                Col {
                    class: "w-full p-4",
                    gap: 2,
                    Row {
                        class: "w-full items-center",
                        div {
                            class: "flex-grow w-full overflow-hidden mr-2",
                            input {
                                class: format!("h-8 outline-none w-full overflow-x-auto {}",
                                    if url_err.read().is_some() {
                                        "text-red-500"
                                    } else if url.read().is_empty() {
                                        "text-elements-lowEmphasis"
                                    } else {
                                        "text-elements-highEmphasis"
                                    }),
                                placeholder: "Enter X post URL",
                                value: url.clone(),
                                oninput: move |e: FormEvent| {
                                    let new_value = e.value();
                                    let valid = if let Some(ref username) = username {
                                        (new_value.contains(&format!("x.com/{}/status", username)) || new_value.contains(&format!("twitter.com/{}/status", username)))
                                    } else {
                                        new_value.starts_with("https://x.com/") || new_value.starts_with("https://twitter.com/")
                                    };
                                    if !new_value.is_empty() && !valid {
                                        url_err.set(Some("URL must contain your username and /status".to_string()));
                                    } else {
                                        url_err.set(None);
                                    }
                                    url.set(new_value);
                                },
                            }
                        }
                    }
                }
            }
            button {
                class: "h-12 w-full rounded-full controls-primary transition-all duration-300 ease-in-out hover:not-disabled:scale-105",
                disabled: url.read().is_empty() || url_err.read().is_some(),
                onclick: move |_| async move {
                    let url = url.read();
                    let res = use_gateway().submit_x_post(url.clone()).await;
                    match res {
                        Ok(body) => {
                            log::info!("{}", body);
                        }
                        Err(e) => {
                            log::error!("Error: {:?}", e);
                        }
                    }
                },
                span {
                    class: "mx-auto my-auto font-semibold",
                    "Submit Post"
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct Post {
    id: String,
    text: String,
    score: i32,
    timestamp: String,
    screen_name: String,
    profile_image_url: String,
    like_count: i32,
    retweet_count: i32,
    reply_count: i32,
    impression_count: i32,
    attributed_score: i32,
}

fn format_timestamp(timestamp: &str) -> String {
    println!("Raw timestamp: {}", timestamp);
    let datetime = DateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S %Z")
        .unwrap_or_else(|e| {
            println!("First parse error: {}", e);
            DateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S UTC")
                .unwrap_or_else(|e| {
                    println!("Second parse error: {}", e);
                    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
                })
        })
        .with_timezone(&Local);
    let formatted = datetime.format("%b %d · %I:%M %p").to_string();
    println!("Formatted timestamp: {}", formatted);
    formatted
}

#[component]
fn PostCard(post: Post) -> Element {
    // Dummy data for avatar, name, and handle
    let display_name = post.screen_name;
    let time = format_timestamp(&post.timestamp);

    rsx! {
        Row {
            class: "w-full gap-3 p-4 bg-black elevated elevated-border rounded-xl items-start",
            // Avatar
            img {
                class: "w-10 h-10 rounded-full border border-elements-lowEmphasis/30",
                src: post.profile_image_url,
                alt: "Profile image"
            }
            // Main content
            Col {
                class: "flex-1 gap-1",
                Row {
                    class: "items-center gap-1 justify-between",
                    span {
                        class: "font-bold text-elements-highEmphasis text-base",
                        "@{display_name}"
                    }                    
                    span {
                        class: "text-elements-lowEmphasis text-base mx-1",
                        "·"
                    }
                    span {
                        class: "text-elements-lowEmphasis text-base",
                        "{time}"
                    }
                    span { 
                        class: "text-elements-highEmphasis ml-auto",
                        "Score: {post.attributed_score}" 
                    }
                }
                // Post text
                span {
                    class: "text-elements-highEmphasis text-base block mt-1 mb-2",
                    "{post.text}"
                }
                // Bottom row for actions
                Row {
                    class: "w-full justify-between items-center mt-2 text-elements-lowEmphasis text-sm",
                    span { 
                        class: "flex items-center gap-1", 
                        XCommentIcon { class: Some("w-4 h-4".to_string()) }
                        "{post.reply_count}" 
                    }
                    span { 
                        class: "flex items-center gap-1", 
                        XRetweetIcon { class: Some("w-4 h-4".to_string()) }
                        "{post.retweet_count}" 
                    }
                    span { 
                        class: "flex items-center gap-1", 
                        XLikeIcon { class: Some("w-4 h-4".to_string()) }
                        "{post.like_count}" 
                    }
                    span { 
                        class: "flex items-center gap-1", 
                        XImpressionsIcon { class: Some("w-4 h-4".to_string()) }
                        "{post.impression_count}" 
                    }                    
                }
            }
        }
    }
}

#[component]
fn PostList(user_id: String, screen_name: String, profile_image_url: String) -> Element {
    let posts = use_resource(move || {
        let user_id = user_id.clone();
        async move { use_gateway().get_x_posts_for_user(user_id).await }
    });

    rsx! {
        Col {
            class: "w-full gap-4",
            match posts.read().as_ref() {
                Some(Ok(posts)) if !posts.is_empty() => {
                    rsx! {
                        {posts.iter().map(|post| {
                            rsx! {
                                Fragment {
                                    key: "{post.post_id}",
                                    PostCard { 
                                        post: Post {
                                            id: post.post_id.clone(),
                                            text: post.text.clone(),
                                            score: post.like_count as i32,
                                            timestamp: post.post_created_at.to_string(),
                                            screen_name: screen_name.clone(),
                                            profile_image_url: profile_image_url.clone(),
                                            like_count: post.like_count as i32,
                                            retweet_count: post.retweet_count as i32,
                                            reply_count: post.reply_count as i32,
                                            impression_count: post.impression_count as i32,
                                            attributed_score: post.attributed_score as i32,
                                        }
                                    }
                                }
                            }
                        })}
                    }
                },
                Some(Ok(_)) => {
                    rsx! {
                        Col {
                            class: "w-full items-center justify-center gap-2 p-8 text-elements-lowEmphasis",
                            span {
                                class: "text-lg font-medium",
                                "No posts yet"
                            }
                            span {
                                class: "text-sm",
                                "Your posts will appear here once you submit them"
                            }
                        }
                    }
                },
                Some(Err(err)) => {
                    rsx! {
                        span {
                            class: "text-elements-error",
                            "Error"
                        }
                    }
                },
                None => {
                    rsx! {
                        div {
                            class: "flex justify-center",
                            div {
                                class: "animate-spin rounded-full h-8 w-8 border-b-2 border-elements-highEmphasis"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Dashboard(user_id: String, screen_name: String, profile_image_url: String) -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Creator Dashboard",
                subtitle: "Submit your content and see your earnings."
            }
            AccountSummary { user_id: user_id.clone(), screen_name: screen_name.clone(), profile_image_url: profile_image_url.clone() }
            Col {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                gap: 8,
                PostForm { username: Some("iocozmo".to_string()) }
                Subheading {
                    title: "Your Posts"
                }
                PostList { user_id: user_id.clone(), screen_name: screen_name.clone(), profile_image_url: profile_image_url.clone() }
            }
        }
    }
}
