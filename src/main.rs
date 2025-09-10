use twitter_v2::TwitterApi;
use twitter_v2::authorization::{Oauth2Token, BearerToken};
use twitter_v2::query::{TweetField, UserField, TweetExpansion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let bearer_token = std::env::var("APP_BEARER_TOKEN")
        .expect("Please set the APP_BEARER_TOKEN environment variable");
    let auth = BearerToken::new(bearer_token);
    // Get the original tweet
    // let tweet = TwitterApi::new(auth.clone())
    //     .get_tweet(1964738399933444376)
    //     .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt, TweetField::Text])
    //     .send()
    //     .await?
    //     .into_data()
    //     .expect("this tweet should exist");
    
    // println!("Original Tweet: {:?}", tweet);
    
    // // Add delay to avoid rate limits
    // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // Add delay before search to avoid rate limits
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Method 1: Get quote tweets (tweets that quote the original)
    let quote_tweets = TwitterApi::new(auth.clone())
        .get_tweet_quote_tweets(1964738399933444376)
        .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt, TweetField::Text])
        .user_fields([UserField::Username, UserField::Name])
        .max_results(10)
        .send()
        .await?
        .into_data();
    
    
    // Display quote tweets
    if let Some(quotes) = quote_tweets {
        println!("Quote Tweets ({}):", quotes.len());
        for quote in quotes {
            println!("- @{}: {}", 
                quote.author_id.unwrap_or(0.into()), 
                quote.text
            );
        }
    } else {
        println!("No quote tweets found");
    }
    
    Ok(())
}
