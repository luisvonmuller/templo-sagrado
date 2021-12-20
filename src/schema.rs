table! {
    address (address_id) {
        address_id -> Int4,
        address_number -> Nullable<Varchar>,
        address_street -> Nullable<Varchar>,
        address_city -> Nullable<Varchar>,
        address_state -> Nullable<Varchar>,
        address_country -> Nullable<Varchar>,
        address_postalcode -> Nullable<Varchar>,
        user_id -> Int4,
    }
}

table! {
    attendance (attendance_id) {
        attendance_id -> Int4,
        client_id -> Int4,
        clerk_id -> Int4,
        agreed -> Nullable<Int4>,
    }
}

table! {
    business_day (business_day_id) {
        business_day_id -> Int4,
        business_day_title -> Nullable<Varchar>,
    }
}

table! {
    text_chat_transaction (text_chat_transaction_id) {
        text_chat_transaction_id -> Int4,
        text_chat_transaction_value -> Float8,
        text_chat_transaction_paid_balance -> Nullable<Float8>,
        text_chat_transaction_paid_bonus -> Nullable<Float8>,
        text_chat_transaction_value_pay_off -> Float8 ,
        text_chat_transaction_chat_id -> Int4,
        text_chat_transaction_client_signature -> Nullable<Varchar>,
        text_chat_transaction_clerk_signature -> Nullable<Varchar>,
        text_chat_transaction_client_id -> Int4,
        text_chat_transaction_clerk_id -> Int4,
        text_chat_transaction_creation -> Timestamp,
        text_chat_transaction_update_client_signature -> Nullable<Timestamp>,
        text_chat_transaction_update_clerk_signature ->  Nullable<Timestamp>,
    }
}

table! {
    voice_chat_transaction (voice_chat_transaction_id) {
        voice_chat_transaction_id -> Int4,
        voice_chat_transaction_value -> Float8,
        voice_chat_transaction_value_pay_off -> Float8 ,
        voice_chat_transaction_paid_balance -> Nullable<Float8>,
        voice_chat_transaction_paid_bonus -> Nullable<Float8>,
        voice_chat_transaction_chat_id -> Int4,
        voice_chat_transaction_client_signature -> Nullable<Varchar>,
        voice_chat_transaction_clerk_signature -> Nullable<Varchar>,
        voice_chat_transaction_client_id -> Int4,
        voice_chat_transaction_clerk_id -> Int4,
        voice_chat_transaction_creation -> Timestamp,
        voice_chat_transaction_update_client_signature -> Nullable<Timestamp>,
        voice_chat_transaction_update_clerk_signature ->  Nullable<Timestamp>,
    }
}

table! {
    business_hour_list (business_hour_list_id) {
        business_hour_list_id -> Int4,
        business_hour_list_begin -> Time,
        business_hour_list_end -> Time,
        business_hour_list_status -> Bool,
        business_day_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    call (call_id) {
        call_id -> Int4,
        call_begin_date -> Timestamp,
        call_end_date -> Nullable<Timestamp>,
        user_id -> Int4,
        clerk_id -> Int4,
        call_file -> Nullable<Text>,
    }
}

table! {
    call_email (call_email_id) {
        call_email_id -> Int4,
        call_email_request_title -> Varchar,
        call_email_request_body -> Text,
        call_email_request_date -> Timestamp,
        call_email_request_to_email -> Varchar,
        call_email_response_title -> Nullable<Varchar>,
        call_email_response_body -> Nullable<Text>,
        call_email_response_date -> Nullable<Timestamp>,
        user_id -> Int4,
        clerk_id -> Int4,
    }
}

table! {
    call_type (call_type_id) {
        call_type_id -> Int4,
        call_type_title -> Varchar,
        call_type_value -> Float8,
    }
}

table! {
    chat (chat_id) {
        chat_id -> Int4,
        client_id -> Int4,
        clerk_id -> Int4,
        client_socket -> Varchar,
        clerk_socket -> Varchar,
        init_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
    }
}

table! {
    chat_msg (chat_msg_id) {
        chat_msg_id -> Int4,
        chat_msg_user_id -> Int4,
        chat_msg_body -> Nullable<Text>,
        chat_msg_time -> Timestamp,
        chat_id -> Int4,
    }
}

table! {
    chat_report (chat_report_id) {
        chat_report_id -> Int4,
        chat_report_user_id -> Int4,
        chat_report_reason -> Nullable<Text>,
        chat_msg_time -> Timestamp,
        chat_id -> Int4,
    }
}

table! {
    clerk_bank (clerk_bank_id) {
        clerk_bank_id -> Int4,
        clerk_id -> Int4,
        clerk_bank_name -> Text,
        clerk_bank_account_type -> Varchar,
        clerk_bank_agency_number -> Varchar,
        clerk_bank_acc_number -> Varchar,
        clerk_bank_cpf -> Varchar,
    }
}

table! {
    clerk_info (clerk_info_id) {
        clerk_info_id -> Int4,
        clerk_description -> Nullable<Text>, //Short description
        clerk_info_long_description -> Nullable<Text>, //Longe description
        clerk_info_experience -> Nullable<Text>, // Experience detailment
        clerk_image -> Nullable<Text>,       // Profile Picture
        clerk_info_cpf -> Nullable<Varchar>,    // UNI
        clerk_info_phrase -> Nullable<Text>,
        clerk_info_comission_rate -> Nullable<Varchar>,
        clerk_info_chat ->  Nullable<Bool>,
        clerk_info_mail -> Nullable<Bool>,
        clerk_info_voice -> Nullable<Bool>,
        clerk_info_webcam -> Nullable<Bool>,
        clerk_info_exhibition ->  Nullable<Varchar>, // Name to be displayed
        clerk_info_priority -> Nullable<Int4>,      //Priority selector
        user_id -> Int4,
    }
}

table! {
    clerk_review (clerk_review_id) {
        clerk_review_id -> Int4,
        clerk_review_title -> Varchar,
        clerk_review_stars -> Int2,
        clerk_review_body -> Nullable<Text>,
        clerk_review_is_anonymous -> Bool,
        user_id -> Int4,
    }
}

table! {
    clerk_skill_list (clerk_skill_list_id) {
        clerk_skill_list_id -> Int4,
        clerk_skill_list_level -> Nullable<Int2>,
        clerk_skill_list_status -> Bool,
        clerk_skill_list_description -> Nullable<Text>,
        user_id -> Int4,
        skill_id -> Int4,
    }
}

table! {
    config (config_id) {
        config_id -> Int4,
        site_name -> Text,
        site_seo_desc -> Text,
        site_seo_tags -> Text,
        site_mail_val -> Float8,
        site_new_user_bonus -> Text,
        absolute_min_value_chat -> Float8,
        absolute_min_value_voice -> Float8,
    }
}

table! {
    end_call_type (end_call_type_id) {
        end_call_type_id -> Int4,
        end_call_type_title -> Varchar,
    }
}

table! {
    message (message_id) {
        message_id -> Int4,
        clerk_info_id -> Int4,
        user_id -> Int4,
        message_header -> Varchar,
        message_body -> Text,
    }
}

table! {
    payment (payment_id) {
        payment_id -> Int4,
        payment_value -> Float8,
        payment_status -> Bool,
        payment_date -> Timestamp,
        payment_obs -> Text,
        user_id -> Int4,
    }
}

table! {
    payment_info (payment_info_id) {
        payment_info_id -> Int4,
        payment_info_bank_number -> Varchar,
        payment_info_bank_name -> Varchar,
        payment_info_account -> Varchar,
        payment_info_cpf -> Varchar,
        payment_info_account_type -> Varchar,
        payment_info_favorecido -> Varchar,
        user_id -> Int4,
    }
}

table! {
    payment_source (payment_source_id) {
        payment_source_id -> Int4,
        payment_source_value -> Float8,
        payment_source_status -> Bool,
        payment_type_id -> Int4,
        payment_id -> Int4,
    }
}

table! {
    payment_type (payment_type_id) {
        payment_type_id -> Int4,
        payment_type_title -> Varchar,
        payment_type_incoming -> Bool,
    }
}

table! {
    phone (phone_id) {
        phone_id -> Int4,
        phone_number -> Varchar,
        user_id -> Int4,
        phone_type_id -> Int4,
    }
}

table! {
    phone_type (phone_type_id) {
        phone_type_id -> Int4,
        phone_type_title -> Nullable<Varchar>,
    }
}

table! {
    post (post_id) {
        post_id -> Int4,
        post_title -> Text,
        post_image -> Text,
        post_seo_tags -> Text,
        post_seo_desc -> Text,
        post_content -> Text,
        post_date -> Date,
    }
}

table! {
    product (product_id) {
        product_id -> Int4,
        product_title -> Varchar,
        product_value -> Float8,
        product_bonus -> Float8,
        product_description -> Text,
        product_image -> Text,
        product_is_active -> Bool,
    }
}

table! {
    product_category (product_category_id) {
        product_category_id -> Int4,
        product_category_title -> Varchar,
    }
}

table! {
    product_list (product_list_id) {
        product_list_id -> Int4,
        product_list_amount -> Int4,
        product_list_use_points -> Bool,
        product_id -> Int4,
        sale_id -> Int4,
    }
}

table! {
    product_review (product_review_id) {
        product_review_id -> Int4,
        products_review_title -> Varchar,
        product_revie_stars -> Int2,
        products_review_body -> Nullable<Text>,
        product_review_is_anonymous -> Bool,
        product_review_date -> Nullable<Timestamp>,
        product_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    sale (sale_id) {
        sale_id -> Int4,
        sale_date -> Timestamp,
        sale_real_value -> Nullable<Float8>,
        sale_points_value -> Nullable<Int4>,
        user_id -> Int4,
        sale_status -> Nullable<Int4>,
        sale_payment_source -> Nullable<Varchar>,
    }
}

table! {
    skill (skill_id) {
        skill_id -> Int4,
        skill_title -> Varchar,
        skill_description -> Nullable<Text>,
        skill_image -> Text,
        skill_status -> Nullable<Bool>,
    }
}

table! {
    status_clerk (status_clerk_id) {
        status_clerk_id -> Int4,
        clerk_id -> Int4,
        status -> Int4,
        is_available_chat -> Bool,
        is_available_voice -> Bool,
        is_available_video -> Bool,
        is_available_mail -> Bool,
    }
}

table! {
    syslayout_item (syslayout_item_id) {
        syslayout_item_id -> Int4,
        syspage_content -> Text,
        syspage_id -> Int4,
    }
}

table! {
    syspage (syspage_id) {
        syspage_id -> Int4,
        syspage_title -> Text,
        syspage_content -> Text,

    }
}

table! {
    sysuser (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        user_email -> Varchar,
        user_password -> Varchar,
        user_birthdate -> Date,
        user_genre -> Varchar,
        user_alias -> Nullable<Varchar>,
        user_newsletter -> Bool,
        user_creation -> Timestamp,
        user_lasttimeonline -> Nullable<Timestamp>,
        user_balance -> Float8,
        user_bonus -> Float8,
        user_type_id -> Int4,
        user_status -> Nullable<Bool>,
        user_uni -> Nullable<Varchar>,
        user_fb_id -> Nullable<Varchar>,
    }
}

table! {
    testimonials (testimonials_id) {
        testimonials_id  -> Int4,
        testimonials_clerk_id -> Int4,
        testimonials_client_id  -> Int4,
        testimonials_content -> Text,
        testimonials_value  -> Int4,
        testimonials_date -> Date,
        testimonials_status -> Bool,
    }
}

table! {
    user_type (user_type_id) {
        user_type_id -> Int4,
        user_type_title -> Varchar,
    }
}

table! {
    stripe_payment (stripe_payment_id) {
        stripe_payment_id -> Int4,
        stripe_payment_source -> Varchar,
        sale_id -> Int4,
    }
}

table! {
    email_notification_list(email_notification_id) {
        email_notification_id -> Int4,
        client_id -> Int4,
        clerk_id -> Int4,
    }
}

table! {
    intends(intend_id) {
        intend_id -> Int4,
        intend_clerk_id -> Int4,
        intend_client_id -> Int4,
        intend_status -> Int4,
        intend_type -> Int4,
        intend_ask_time -> Timestamp,
        intend_received_time -> Nullable<Timestamp>,
        intend_answer_time -> Nullable<Timestamp>,
    }
}

table! {
    banners (banner_id) {
        banner_id -> Int4,
        banner_creation_date -> Timestamp,
        banner_mobile -> Text,
        banner_desktop -> Text,
    }
}

table! {
    syslog (syslog_id) {
        syslog_id -> Int4,
        syslog_creation -> Timestamp,
        syslog_content -> Text,
    }
}

table! {
    clerk_time(clerk_time_id) {
        clerk_time_id -> Int4,
        clerk_time_clerk_id -> Int4,
        clerk_time_date -> Timestamp,
        clerk_time_event_type -> Int4,
    }
}

table! {
    clerk_schedule(clerk_schedule_id) {
        clerk_schedule_id -> Int4,
        clerk_schedule_user_id -> Int4,
        clerk_schedule_mon -> Bool,
        clerk_schedule_mon_init -> Varchar,
        clerk_schedule_mon_end -> Varchar,
        clerk_schedule_tue -> Bool,
        clerk_schedule_tue_init -> Varchar,
        clerk_schedule_tue_end -> Varchar,
        clerk_schedule_wed -> Bool,
        clerk_schedule_wed_init -> Varchar,
        clerk_schedule_wed_end -> Varchar,
        clerk_schedule_thu -> Bool,
        clerk_schedule_thu_init -> Varchar,
        clerk_schedule_thu_end -> Varchar,
        clerk_schedule_fri -> Bool,
        clerk_schedule_fri_init -> Varchar,
        clerk_schedule_fri_end -> Varchar,
        clerk_schedule_sat -> Bool,
        clerk_schedule_sat_init -> Varchar,
        clerk_schedule_sat_end -> Varchar,
        clerk_schedule_sun -> Bool,
        clerk_schedule_sun_init -> Varchar,
        clerk_schedule_sun_end -> Varchar,
    }
}

table! {
    attendance_tag(attendance_tag_id) {
        attendance_tag_id -> Int4,
        attendance_tag_name -> Varchar,
        attendance_tag_slug -> Varchar,
    }
}

table! { 
    paypal_payment(paypal_payment_id) { 
        paypal_payment_id -> Int4, 
        paypal_payment_source_identifier -> Text, 
        paypal_payment_sale_id -> Int4, 
    }
}

table! {
    clerk_tag(clerk_tag_id) {
        clerk_tag_id -> Int4,
        clerk_tag_user_id -> Int4,
        clerk_tag_attendance_tag_id -> Int4,
    }
}

table! {
    global_states(global_states_id) {
        global_states_id -> Int4,
        voice_minutes -> Int4,
    }
}


joinable!(address -> sysuser (user_id));
joinable!(business_hour_list -> business_day (business_day_id));
joinable!(business_hour_list -> sysuser (user_id));
joinable!(call -> sysuser (user_id));

joinable!(clerk_tag -> attendance_tag(clerk_tag_attendance_tag_id));
joinable!(clerk_schedule -> sysuser (clerk_schedule_user_id));
joinable!(chat -> sysuser (client_id));
joinable!(call_email -> sysuser (user_id));
joinable!(chat_msg -> sysuser (chat_msg_user_id));
joinable!(chat_report -> sysuser (chat_report_user_id));
joinable!(clerk_bank -> sysuser (clerk_id));
joinable!(clerk_info -> sysuser (user_id));
joinable!(clerk_review -> sysuser (user_id));
joinable!(clerk_skill_list -> skill (skill_id));
joinable!(clerk_skill_list -> sysuser (user_id));
joinable!(message -> clerk_info (clerk_info_id));
joinable!(message -> sysuser (user_id));
joinable!(payment -> sysuser (user_id));
joinable!(payment_info -> sysuser (user_id));
joinable!(payment_source -> payment (payment_id));
joinable!(payment_source -> payment_type (payment_type_id));
joinable!(phone -> phone_type (phone_type_id));
joinable!(phone -> sysuser (user_id));
joinable!(product_list -> product (product_id));
joinable!(product_list -> sale (sale_id));
joinable!(product_review -> product (product_id));
joinable!(product_review -> sysuser (user_id));
joinable!(sale -> sysuser (user_id));
joinable!(status_clerk -> sysuser (clerk_id));
joinable!(syslayout_item -> syspage (syspage_id));
joinable!(sysuser -> user_type (user_type_id));

allow_tables_to_appear_in_same_query!(
    address,
    attendance,
    business_day,
    business_hour_list,
    attendance_tag,
    clerk_tag,
    call,
    call_email,
    call_type,
    chat,
    chat_msg,
    chat_report,
    clerk_bank,
    clerk_info,
    clerk_review,
    clerk_skill_list,
    config,
    end_call_type,
    message,
    payment,
    payment_info,
    paypal_payment,
    payment_source,
    payment_type,
    phone,
    phone_type,
    post,
    product,
    product_category,
    product_list,
    product_review,
    sale,
    skill,
    status_clerk,
    syslayout_item,
    syspage,
    sysuser,
    user_type,
    intends,
);
